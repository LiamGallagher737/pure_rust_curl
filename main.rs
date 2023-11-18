#![no_std]
#![no_main]
#![allow(dead_code)]

use core::panic::PanicInfo;

mod syscalls;

#[no_mangle]
pub extern "C" fn _start() {
    let msg = b"Hello, world!\n";
    let res = syscalls::write(1, msg.as_ptr(), msg.len() as u64);
    syscalls::exit(res);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
