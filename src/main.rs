#![feature(test)]
#![feature(iter_next_chunk)]

extern crate test;
mod asm_metadata;
use std::io;
use std::io::prelude::*;
use std::fs::File;

use asm_metadata::TARGET_MACHINE_TYPES;

fn main() -> io::Result<()> {
    let file = File::open(r"D:\Games\AssaultCube\bin_win32\ac_client.exe").expect("Couldn't read file");

    println!("\n--- HEX DUMP ---\n");

    let mut byte_iter = file.bytes();

    let mut buf = [0u8; 2];
    match (&mut byte_iter).nth(0x3C) {
        Some(byte) =>  {
            buf[0] = byte.expect("Failed to read PE Signature - first byte missing");
            buf[1] = byte_iter.next().expect("Unexpected EOF while finding PE Signature").expect("Failed to read PE Signature - second byte missing");
        },
        _ => panic!("Failed to find PE Signature offset!"),
    }

    let pe_signature_offset = u16::from_ne_bytes(buf);
    let pe_signature_address = pe_signature_offset - 0x3C - 2;

    let mut buf = [0u8; 4];
    match (&mut byte_iter).nth(pe_signature_address.into()) {
        Some(byte) =>  {
            buf[0] = byte.expect("Failed to read PE Signature - first byte missing");
            buf[1] = byte_iter.next().expect("Unexpected EOF while finding PE Signature").expect("Failed to read PE Signature - second byte missing");
            buf[2] = byte_iter.next().expect("Unexpected EOF while finding PE Signature").expect("Failed to read PE Signature - third byte missing");
            buf[3] = byte_iter.next().expect("Unexpected EOF while finding PE Signature").expect("Failed to read PE Signature - fourth byte missing");
        },
        _ => panic!("Failed to find PE Signature!"),
    }

    let pe_signature = std::str::from_utf8(&buf).expect("Failed to convert PE Signature bytes to utf-8");
    print!("Found PE Signature: {:X?}\n", pe_signature);

    let mut buf = [0u8; 2];
    match (&mut byte_iter).next() {
        Some(byte) =>  {
            buf[0] = byte.expect("Failed to read target machine type - first byte missing");
            buf[1] = byte_iter.next().expect("Unexpected EOF while finding target machine type").expect("Failed to read target machine type - second byte missing");
        },
        _ => panic!("Failed to find target machine type!"),
    }

    let target_machine_type = TARGET_MACHINE_TYPES.get(&u16::from_ne_bytes(buf));
    print!("Target machine: {:X?} ({})\n", u16::from_ne_bytes(buf), target_machine_type.unwrap());


    Ok(())
}
