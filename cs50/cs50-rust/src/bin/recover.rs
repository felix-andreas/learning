use std::fs::File;
use std::io::prelude::*;

fn check_jpg(buffer: &[u8]) -> bool {
    buffer[0] == 0xff && buffer[1] == 0xd8 && buffer[2] == 0xff && (buffer[3] & 0xf0) == 0xe0
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut raw_file = File::open(&args[1]).unwrap();
    let mut buffer = [0; 512];
    for photo in 0..1000 {
        let mut image_file = File::create(format!("output/{:03}.jpg", photo)).unwrap();
        println!("recover file: {}", photo);

        while !check_jpg(&buffer) {
            raw_file.read_exact(&mut buffer).unwrap();
        }

        loop {
            image_file.write_all(&buffer).unwrap();
            if let Err(_e) = raw_file.read_exact(&mut buffer) {
                return;
            }

            if check_jpg(&buffer) {
                break;
            }
        }
    }
}
