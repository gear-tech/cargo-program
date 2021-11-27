#![no_std]

use gstd::msg;

#[gstd::async_main]
async fn main() {
    msg::reply(b"Hello world!", 0, 0);
}

#[no_mangle]
pub unsafe extern "C" fn init() {}
