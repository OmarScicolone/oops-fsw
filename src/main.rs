// Entry point: loads the raw byte stream from disk, runs the parser, and prints results.

mod packet;
mod checksum;
mod parser;

use parser::parse;
use packet::InvalidReason;

fn main() {
    let bytes = load_files("data/fsw-output.txt");

    let result = parse(&bytes);

    // Q1: how many valid packets?
    println!("Q1 - Valid packets: {}", result.valid.len());

    // Q2: how many invalid packets, and why?
    let mut n_bad_type      = 0;
    let mut n_bad_length    = 0;
    let mut n_bad_device    = 0;
    let mut n_bad_direction = 0;
    let mut n_bad_checksum  = 0;

    for (reason, _) in &result.invalid {
        match reason {
            InvalidReason::BadPacketType      => n_bad_type      += 1,
            InvalidReason::BadDataLength      => n_bad_length    += 1,
            InvalidReason::BadDeviceId        => n_bad_device    += 1,
            InvalidReason::BadPacketDirection => n_bad_direction += 1,
            InvalidReason::BadChecksum        => n_bad_checksum  += 1,
        }
    }

    println!("Q2 - Invalid packets: {}", result.invalid.len());
    println!("  BadPacketType:      {}", n_bad_type);
    println!("  BadDataLength:      {}", n_bad_length);
    println!("  BadDeviceId:        {}", n_bad_device);
    println!("  BadPacketDirection: {}", n_bad_direction);
    println!("  BadChecksum:        {}", n_bad_checksum);

    // Q3: how many padding bytes?
    println!("Q3 - Padding bytes: {}", result.padding_bytes);
}

fn load_files(path_file: &str) -> Vec<u8> {
    let content = std::fs::read_to_string(path_file).expect("file not found");
    let mut bytes = Vec::new();
    
    for token in content.split_whitespace() {
        let byte = u8::from_str_radix(token, 16).expect("invalid token");
        bytes.push(byte);
    }
    bytes
}
