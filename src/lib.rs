#![no_std]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]

extern crate uefi;
extern crate rlibc;
extern crate compiler_builtins;

use uefi::SimpleTextOutput;

#[allow(unreachable_code)]
#[no_mangle]
pub extern "win64" fn efi_main(hdl: uefi::Handle, sys: uefi::SystemTable) -> uefi::Status {
    uefi::initialize_lib(&hdl, &sys);
    let console = uefi::get_system_table().console();
    console.write("Hello!");
    loop {
    }
    uefi::Status::Success
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    loop {}
}
