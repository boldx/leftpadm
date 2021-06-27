#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use linux_kernel_module::{self, cstr, Error};
//use linux_kernel_module::println;
use linux_kernel_module::sync::Spinlock;
use lazy_static::*;

lazy_static! {
    static ref BUFF: Spinlock<Vec<u8>> = Spinlock::new(Vec::<u8>::new());
}

struct LeftPadFile;

impl linux_kernel_module::file_operations::FileOperations for LeftPadFile {

    fn open() -> linux_kernel_module::KernelResult<Self> {
        Ok(LeftPadFile {})
    }

    const READ: linux_kernel_module::file_operations::ReadFn<Self> = Some(
        |_this: &Self,
         _file: &linux_kernel_module::file_operations::File,
         buf: &mut linux_kernel_module::user_ptr::UserSlicePtrWriter,
         offset: u64|
         -> linux_kernel_module::KernelResult<()> {
            if offset == 0 { 
                //let val = unsafe { BUFF.clone() };
                let val = (*BUFF.lock()).clone();
                buf.write(&val)?;
            }
            Ok(())
        },
    );

    const WRITE: linux_kernel_module::file_operations::WriteFn<Self> = Some(
        |_this: &Self,
         buf: &mut linux_kernel_module::user_ptr::UserSlicePtrReader,
         _offset: u64|
         -> linux_kernel_module::KernelResult<()> {
            let data = buf.read_all()?;
            
            let params = String::from_utf8(data).unwrap_or(String::new());
            let params = params.splitn(3, " ").collect::<Vec<&str>>();
            if params.len() != 3 {
                return Err(Error::EINVAL)
            }
            
            let target_len = params[0].parse::<usize>().unwrap_or(0);
            let mut pad_unit = params[1];
            let mut str_to_pad = params[2];
            if pad_unit.is_empty() {
                pad_unit = " ";
                str_to_pad = &str_to_pad[1..];
            }
            
            let pad_len = target_len.checked_sub(str_to_pad.chars().count()).unwrap_or(0);
            let mut pad_str = String::new();
            let mut pad_unit_it = pad_unit.chars().cycle();
            while pad_str.chars().count() < pad_len {
                match pad_unit_it.next() {
                    Some(char) => pad_str.push(char),
                    None => break,
                }
            }
            let padded_str = pad_str + str_to_pad;
/*
            unsafe { 
                BUFF.clear();
                BUFF.extend_from_slice(padded_str.as_bytes());
            }; 
*/          let mut buff = BUFF.lock();
            (*buff).clear();
            (*buff).extend_from_slice(padded_str.as_bytes());
            Ok(())
        },
    );
}


struct LeftPadModule {
    _chrdev_registration: linux_kernel_module::chrdev::Registration,
}

impl linux_kernel_module::KernelModule for LeftPadModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let chrdev_registration =
            linux_kernel_module::chrdev::builder(cstr!("leftpad"), 0..1)?
                .register_device::<LeftPadFile>()
                .build()?;
        Ok(LeftPadModule {
            _chrdev_registration: chrdev_registration,
        })
    }
}


linux_kernel_module::kernel_module!(
    LeftPadModule,
    author: b"boldx",
    description: b"A module for left-padding strings",
    license: b"GPL"
);
