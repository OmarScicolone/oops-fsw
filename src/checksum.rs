// Checksum algorithm for OOPS packets (16-bit accumulator, init 0xACE1).
// Applied to all bytes between the sync marker and the checksum field (exclusive).

pub fn compute_checksum(bytes: &[u8]) -> u16 {
    let mut acc: u16 = 0xACE1;

    for &byte in bytes {
        acc = acc.rotate_left(3);
        acc = acc ^ byte as u16;
        acc = acc.wrapping_add(0x1F);
    }
    acc
}
