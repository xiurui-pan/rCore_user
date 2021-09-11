#![feature(linkage)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![no_std]
#![allow(dead_code)]
#![allow(deprecated)]

#[macro_use]
pub mod console;
mod syscall;
mod lang_items;

pub const STDOUT: usize = 1;

use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }

pub fn exit(exit_code: i32) -> isize { sys_exit(exit_code) }

pub fn yield_() -> isize { sys_yield() }

pub fn get_time() -> isize {
    let time = TimeVal::new();
    match sys_get_time(&time, 0) {
        0 => ((time.sec & 0xffff) * 1000 + time.usec / 1000) as isize,
        _ => -1,
    }
}

pub fn sleep(period_ms: usize) {
    let start = get_time();
    while get_time() < start + period_ms as isize {
        sys_yield();
    }
}

pub fn set_priority(prio: isize) -> isize {
    sys_set_priority(prio)
}

pub fn wait(exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(-1, exit_code as *mut _) {
            -2 => { sys_yield(); }
            n => { return n ; }
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

impl TimeVal {
    pub fn new() -> Self {
        TimeVal { sec: 0, usec: 0 }
    }
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize .. ebss as usize).for_each(|a|{
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
