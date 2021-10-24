use anyhow::Result;
use anyhow::*;
use directories::ProjectDirs;
use lazy_static::*;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fs::{write, File};
use std::io::BufReader;

lazy_static! {
    static ref STORAGE: Mutex<HashMap<String, String>> = Mutex::new(
        load().unwrap_or_else(|_| panic!("could not load local storage from {:?}", get_path()))
    );
}

pub fn get_item(key: &str) -> Result<Option<String>> {
    let s = STORAGE.lock();
    match s.get(key) {
        Some(txt) => Ok(Some(txt.to_string())),
        None => Ok(None),
    }
}

pub fn remove_item(key: &str) -> Result<Option<String>> {
    let mut s = STORAGE.lock();
    let r = match s.remove(key) {
        Some(txt) => Ok(Some(txt)),
        None => Ok(None),
    };
    save(&s)?;
    r
}

pub fn set_item(key: &str, value: &str) -> Result<()> {
    let mut s = STORAGE.lock();
    s.insert(key.to_string(), value.to_string());
    save(&s)?;
    Ok(())
}

fn get_project_path() -> Result<String> {
    let dir = if let Some(proj_dirs) = ProjectDirs::from("", "", env!("CARGO_PKG_NAME")) {
        if let Some(s) = proj_dirs.config_dir().to_str() {
            s.to_string()
        } else {
            ".".to_string()
        }
    } else {
        ".".to_string()
    };
    let p = std::path::Path::new(&dir);
    let p = p.join("local_storage.json");
    let s = p.to_str().unwrap().to_string();
    Ok(s)
}

fn get_path() -> String {
    get_project_path().unwrap_or("local_storage.json".to_string())
}

fn load() -> Result<HashMap<String, String>> {
    let fp = &get_path();
    if !std::path::Path::new(fp).exists() {
        return Ok(HashMap::new());
    }
    let file = File::open(fp)?;
    let reader = BufReader::new(file);

    let u: HashMap<String, String> = serde_json::from_reader(reader)?;
    Ok(u)
}

fn save(h: &HashMap<String, String>) -> Result<()> {
    let fp = &get_path();
    std::fs::create_dir_all(std::path::Path::new(fp).parent().unwrap())?;
    let txt = serde_json::to_string(h)?;
    write(fp, txt)?;
    Ok(())
}

pub fn clear() -> Result<()> {
    let mut s = STORAGE.lock();
    s.clear();
    Ok(())
}

pub fn length() -> Result<usize> {
    let s = STORAGE.lock();
    save(&s)?;
    Ok(s.len())
}

pub fn key(index: usize) -> Result<Option<String>> {
    let s = STORAGE.lock();
    let mut vals = s.values();
    let v = vals.nth(index);
    match v {
        Some(key) => match s.get(key) {
            Some(txt) => Ok(Some(txt.to_string())),
            None => Ok(None),
        },
        None => Err(anyhow!("index out of bounds")),
    }
}

pub fn location() -> String {
    get_path()
}
