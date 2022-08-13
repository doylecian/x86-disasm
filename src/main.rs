#![feature(test)]

extern crate test;
mod asm_opcodes;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open(r"D:\Games\AssaultCube\bin_win32\ac_client.exe").expect("Couldn't read file");

    println!("\n--- HEX DUMP ---\n");
    println!("\n\t[PE Header]\n");

    let mut file_iter = file.bytes();

    let mut byte_count = 0x1;

    for byte in (&mut file_iter).take(0x400) {
        print!("{:X} ", byte.unwrap());
        if byte_count % 0x10 == 0 {
            println!("\t{:#010X}\n", byte_count);
        }
        byte_count += 1;
    }

    println!("\n\t[Raw Data]\n");

    let mut byte_count = 0x1;

    for byte in file_iter {
        print!("{:X} ", byte.unwrap());
        if byte_count % 0x10 == 0 {
            println!("\t{:#010X}\n", (byte_count + 0x400) as usize);
        }
        if byte_count == 0x500 {
            return Ok(())
        }
        byte_count += 1;
    }

    Ok(())
}
