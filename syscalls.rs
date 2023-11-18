use core::arch::asm;

macro_rules! syscall {
    ($call:expr, $arg1:expr) => {{
        let res: i64;
        asm!(
            "syscall",
            in("rax") $call as u64,
            in("rdi") $arg1,
            lateout("rax") res,
        );
        res
    }};
    ($call:expr, $arg1:expr, $arg2:expr) => {{
        let res: i64;
        asm!(
            "syscall",
            in("rax") $call as u64,
            in("rdi") $arg1,
            in("rsi") $arg2,
            lateout("rax") res,
        );
        res
    }};
    ($call:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {{
        let res: i64;
        asm!(
            "syscall",
            in("rax") $call as u64,
            in("rdi") $arg1,
            in("rsi") $arg2,
            in("rdx") $arg3,
            lateout("rax") res,
        );
        res
    }};
}

#[repr(u64)]
pub enum SysCall {
    Read = 0,
    Write = 1,
    Exit = 60,
}

pub fn read(fd: u64, buf: *mut u8, size: u64) -> i64 {
    unsafe { syscall!(SysCall::Read, fd, buf as u64, size as u64) }
}

pub fn write(fd: u64, data: *const u8, len: u64) -> i64 {
    unsafe { syscall!(SysCall::Write, fd, data as u64, len) }
}

pub fn exit(code: i64) -> i64 {
    unsafe { syscall!(SysCall::Exit, code) }
}
