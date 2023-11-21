#![no_std]
#![no_main]
#![allow(dead_code)]

#[cfg(any(not(target_os = "linux"), not(target_arch = "x86_64")))]
compile_error!("Only works on x86_64 Linux");

use core::panic::PanicInfo;
use core::arch::asm;
use syscalls::SysFd;

mod syscalls;

#[no_mangle]
pub unsafe extern "C" fn _start() {
    asm!("mov r8, rsp");
    let stack_ptr: *const usize;
    asm!("mov {}, r8", out(reg) stack_ptr);
    let args_ptr = stack_ptr.add(3);
    let argc = *args_ptr;

    if argc < 2 {
        println("ERROR: Must have a url argument - curl [url]");
        syscalls::exit(1);
    }

    let url_arg_ptr = args_ptr.add(2);
    let url = *url_arg_ptr as *const u8;

    let code = main(url);

    syscalls::exit(code);
}

unsafe fn main(url: *const u8) -> i64 {
    let mut c = *url;
    let mut n = 0;
    while c != 0 {
        syscalls::write(SysFd::Stdout, &c, 1);
        n += 1;
        c = *url.add(n)
    }
    println("");
    0
}

fn print(msg: &str) -> i64 {
    syscalls::write(SysFd::Stdout, msg.as_ptr(), msg.len() as u64)
}

fn println(msg: &str) -> i64 {
    print(msg);
    syscalls::write(SysFd::Stdout, b"\n".as_ptr(), 1)
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let msg = b"ERROR\n";
    syscalls::write(SysFd::Stdout, msg.as_ptr(), msg.len() as u64);
    syscalls::exit(1);
    loop {}
}
