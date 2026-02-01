mod client;

pub use client::*;

use gloo_storage::{LocalStorage, Storage};

const TOKEN_KEY: &str = "auth_token";

pub fn get_token() -> Option<String> {
    LocalStorage::get(TOKEN_KEY).ok()
}

pub fn set_token(token: &str) {
    let _ = LocalStorage::set(TOKEN_KEY, token);
}

pub fn remove_token() {
    LocalStorage::delete(TOKEN_KEY);
}
