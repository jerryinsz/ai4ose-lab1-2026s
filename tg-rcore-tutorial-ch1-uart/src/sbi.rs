#![allow(unused)]

use core::arch::asm;

const SBI_SYSTEM_RESET: usize = 0x53525354;

#[inline(always)]
fn sbi_call(extension: usize, function: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x16") function,
            in("x17") extension,
        );
    }
    ret
}

pub fn shutdown(failure: bool) -> ! {
    let status = if failure { 1 } else { 0 };
    let reason = if failure { 1 } else { 0 };
    sbi_call(SBI_SYSTEM_RESET, 0, status, reason, 0);
    unreachable!()
}
