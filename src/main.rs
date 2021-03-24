#![no_std]
#![no_main]
#![feature(asm)]
#![feature(naked_functions)]

mod reset;

use core::panic::PanicInfo;

extern "C" {
    static __bootflash_start__: u32;
    static __bootflash_size__: u32;
    static __appflash_start__: u32;
    static __appflash_size__: u32;
    static __eram_start__: u32;
    static __eram_size__: u32;
}

#[naked]
unsafe extern "C" fn start_app(_pc: u32, _sp: u32) {
    asm!(
        "msr msp, r1 /* load r1 into MSP */",
        "bx r0 /* branch to the address at r0 */",
        options(noreturn)
    );
}

#[allow(unreachable_code)]
pub fn main() -> ! {
    unsafe {
        let app_code: *mut u32 = __appflash_start__ as *mut u32;
        let dst: *mut u32 = __eram_start__ as *mut u32;
        let size: usize = __appflash_size__ as usize;
        core::ptr::copy_nonoverlapping(app_code, dst, size);

        let app_sp: u32 = *dst;
        let app_start: u32 = *dst.offset(1);

        start_app(app_start, app_sp);
    }
    unreachable!(loop {});
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        continue;
    }
}
