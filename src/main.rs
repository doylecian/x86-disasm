#![feature(test)]

extern crate test;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use phf::phf_map;

fn conv_map(digit: u8) -> String {
    match digit {
        0xE9 => "jmp".to_string(),
        0x90 => "nop".to_string(),
        0x50 => "push eax".to_string(),
        0x51 => "push ecx".to_string(),
        0x52 => "push edx".to_string(),
        0x53 => "push ebx".to_string(),
        0x54 => "push esp".to_string(),
        0x55 => "push ebp".to_string(),
        0x56 => "push esi".to_string(),
        0x57 => "push edi".to_string(),
        0x58 => "pop eax".to_string(),
        0x59 => "pop ecx".to_string(),
        _   => "NA".to_string(),
    }
}

static OPCODES: phf::Map<u8, &'static str> = phf_map! {
    0xE9u8 => "dw3da1d",
    0x90u8 => "dw12dad",
    0x50u8 => "push eax,",
    0x51u8 => "push ecx",
    0x52u8 => "push edx",
    0x53u8 => "push ebx",
    0x54u8 => "push esp",
    0x55u8 => "push ebp",
    0x56u8 => "push esi",
    0x57u8 => "push edi",
    0x58u8 => "pop eax",
    0x59u8 => "push ecx",
    0x5Au8 => "push edx",
    0x5Bu8 => "push ebx",
    0x5Cu8 => "push esp",
    0x5Du8 => "push ebp",
    0x5Eu8 => "push esi",
    0x5Fu8 => "push edi",
};


fn main() -> io::Result<()> {
    let mut file = File::open(r"D:\Games\AssaultCube\bin_win32\ac_client.exe").expect("Couldn't read file");

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


#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};

    use crate::{conv_map, OPCODES};

    #[bench]
    fn bench_phf_hash_map(b: &mut Bencher) {
        b.iter(|| {
            for i in  0..0xd9000 {
                black_box(OPCODES.get(&(0x59 as u8)));
            }
        });
    }

    #[bench]
    fn bench_match_table(b: &mut Bencher) {
        b.iter(|| {
            for i in  0..0xd9000 {
                let x = conv_map(0x59);
            }
        });
    }
}