fn main() {
    web_local_storage_api::set_item("foo", "bar").unwrap();
    let s = web_local_storage_api::get_item("foo");
    println!("hey {:?} {:?}", &s, web_local_storage_api::location());
}
