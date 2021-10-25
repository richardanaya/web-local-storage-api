# Rust Web Local Storage API

[![crates.io](https://img.shields.io/crates/v/web-local-storage-api.svg)](https://crates.io/crates/web-local-storage-api)

A Rust implementation of the Web LocalStorage API, for use in non-browser contexts

## About the Web Local Storage API

[MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)

The Web LocalStorage API provides a simple way for storing string key value pairs for your app.  These values will be stored in your 
operating systemz appropriate configuration location based off your project name.

```
// Linux:   /home/alice/.config/barapp
// Windows: C:\Users\Alice\AppData\Roaming\barapp
// macOS:   /Users/Alice/Library/Application Support/barapp
```

## Usage

[Docs](https://docs.rs/web-local-storage-api)

```rust
// set a value of the key value pair, this is automatically saved
web_local_storage_api::set_item("foo","bar")?;

// retrieve a key value
web_local_storage_api::get_item("foo")?; // returns `Some("bar")`

// remove the key value
web_local_storage_api::remove_item("foo")?;

// clear all key values
web_local_storage_api::clear()?;
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `web-local-storage-api` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
