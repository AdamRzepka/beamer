#![no_std]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![feature(alloc, global_allocator, allocator_api)]

extern crate uefi;
extern crate rlibc;
extern crate compiler_builtins;
#[macro_use]
extern crate alloc;

use alloc::allocator::{Alloc, Layout, AllocErr};
use alloc::string::*;

struct UefiAllocator;

unsafe impl<'a> Alloc for &'a UefiAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        uefi::get_system_table().console().write("alloc: ");
        match uefi::get_system_table().boot_services().allocate_pool(layout.size() + (1 << 10)) {
            Ok(mem) => {
                uefi::get_system_table().console().write("Success\r\n");
                Ok(mem)
            },
            Err(status) => { 
                uefi::get_system_table().console().write(status.str());
                Err(AllocErr::Exhausted {
                    request: layout,
                })
            }
        }
   }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        
    }
}

#[global_allocator]
static GLOBAL: UefiAllocator = UefiAllocator;

use uefi::SimpleTextOutput;

#[allow(unreachable_code)]
#[no_mangle]
pub extern "win64" fn efi_main(hdl: uefi::Handle, sys: uefi::SystemTable) -> uefi::Status {
    uefi::initialize_lib(&hdl, &sys);

    let console = uefi::get_system_table().console();
    console.write("Hello!\r\n");

    let _test_vec = vec![1, 2, 3];
    console.write("We have vector!\r\n");
    console.write(&alloc::String::from("We have a string!\r\n"));
    // format does not work for now
    // console.write(&format!("{} {} {}\r\n", test_vec[0], test_vec[1], test_vec[2]));
    console.write(&alloc::String::from("End"));
    loop {
    }
    uefi::Status::Success
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    uefi::get_system_table().console().write("panic!");
    loop {}
}
