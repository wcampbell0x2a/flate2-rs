extern crate flate2;

use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::bufread::DeflateEncoder;
use std::fs::File;
use std::io::BufReader;

// Open file and debug print the contents compressed with Deflate
fn main() {
    println!("{:?}", open_hello_world().unwrap());
}

// Opens sample file, compresses the contents and returns a Vector or error
// File wrapped in a BufReader implements Bufread
fn open_hello_world() -> io::Result<Vec<u8>> {
    let f = File::open("examples/hello_world.txt")?;
    let b = BufReader::new(f);
    let mut deflater = DeflateEncoder::new(b, Compression::Fast);
    let mut buffer = Vec::new();
    deflater.read_to_end(&mut buffer)?;
    Ok(buffer)
}