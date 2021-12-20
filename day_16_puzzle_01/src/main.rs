mod bits;
mod hex;
mod packet;

use std::fs;
use crate::packet::Packet;

fn main() -> std::io::Result<()> {
    let hex_data = fs::read_to_string("./day_16_puzzle_01/input.txt").unwrap();

    let binary_data = hex::convert_hex_value_to_binary(hex_data.trim());

    let (packet, _) = bits::extract_packet(&binary_data);

    let version_totals = packet.version_total();

    println!("{}", version_totals);

    Ok(())
}