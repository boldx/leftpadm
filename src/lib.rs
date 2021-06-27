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

fn left_pad(target_char_len: usize, pad_str: &str, str_to_pad: &str) -> String {
    let pad_len = target_char_len.checked_sub(str_to_pad.chars().count()).unwrap_or(0);
    let mut padding = String::new();
    let mut pad_str_it = pad_str.chars().cycle();
    while padding.chars().count() < pad_len {
        match pad_str_it.next() {
            Some(char) => padding.push(char),
            None => break,
        }
    }
    padding + str_to_pad
}

fn parse_args<'a>(arg_str: &'a str) -> Result<(usize, &'a str, &'a str), &'static str> {
    let args = arg_str.splitn(3, " ").collect::<Vec<&str>>();
    if args.len() != 3 {
        return Err("Too few args")
    }
    let target_len = args[0].parse::<usize>().unwrap_or(0);
    let mut pad_str = args[1];
    let mut str_to_pad = args[2];
    if pad_str.is_empty() {
        pad_str = " ";
        str_to_pad = &str_to_pad[1..];
    }
    Ok((target_len, pad_str, str_to_pad))
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
                buf.write(&*(BUFF.lock()))?;
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
            let args = String::from_utf8(data).unwrap_or(String::new());
            let (target_len, pad_unit, str_to_pad) = match parse_args(&args) {
                Ok(t) => t,
                Err(_) => return Err(Error::EINVAL),
            };
            let padded_str = left_pad(target_len, pad_unit, str_to_pad);
            let mut buff = BUFF.lock();
            (*buff).clear();
            (*buff).extend_from_slice(padded_str.as_bytes());
            Ok(())
        },
    );
}


struct LeftPadModule {
    _reg: linux_kernel_module::chrdev::Registration,
}

impl linux_kernel_module::KernelModule for LeftPadModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let _reg =
            linux_kernel_module::chrdev::builder(cstr!("leftpad"), 0..1)?
                .register_device::<LeftPadFile>()
                .build()?;
        Ok(LeftPadModule { _reg })
    }
}


linux_kernel_module::kernel_module!(
    LeftPadModule,
    author: b"boldx",
    description: b"A module for left-padding strings",
    license: b"GPL"
);

