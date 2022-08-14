#![feature(test)]
#![feature(iter_next_chunk)]

extern crate test;
mod asm_metadata;
use std::io::{self, BufReader, BufWriter};
use std::io::prelude::*;
use std::fs::File;

use asm_metadata::TARGET_MACHINE_TYPES;

fn main() -> io::Result<()> {

    let file = File::open(r"D:\Games\AssaultCube\bin_win32\ac_client.exe").expect("Couldn't read file");
    let byte_count = file.metadata().unwrap().len();

    println!("Opened file of size {:X}", byte_count);

    let mut read_buf = BufReader::new(file);
    let output_file = File::create("asm_dump.txt").unwrap();
    let mut write_buf =  BufWriter::new(output_file);
    write_buf.write(b"0x00000000\t\t").expect("Couldn't write bytes to file");

    let mut byte_count = 1;
    for byte in read_buf.by_ref().bytes().by_ref() {
        match byte {
            Ok(byte) => {
                if byte_count % 16 == 0 {
                    write_buf.write_fmt(format_args!("{:#X}\n{:#010X}\t\t", byte, byte_count)).expect("Couldn't write bytes to file");
                }
                else {
                    write_buf.write_fmt(format_args!("{:#X} ", byte)).expect("Couldn't write bytes to file");
                }
                byte_count += 1;
            }
            _ => break
        }
    }

    println!("\n--- HEX DUMP ---\n");
    read_buf.rewind().unwrap();
    let mut byte_iter = read_buf.bytes();

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
