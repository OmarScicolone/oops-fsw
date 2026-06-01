// Scans the raw byte stream for OOPS packets (sync marker A5 F0 EE).
// Extracts, validates, and classifies each packet as valid or invalid.

use crate::packet::{
    Packet, PacketType, InvalidReason,
    SYNC, TERMINATION, MAX_DATA_LEN, MIN_PACKET_LEN,
    OFFSET_TYPE, OFFSET_SRC, OFFSET_DST, OFFSET_RESERVED, OFFSET_DATA_LEN, OFFSET_DATA,
    CHECKSUM_LEN, CONTROLLER_ID, OOPS_IDS, BROADCAST_ID,
};
use crate::checksum::compute_checksum;

pub struct ParseResult {
    pub valid: Vec<Packet>,
    pub invalid: Vec<(InvalidReason, Option<Packet>)>,
    pub padding_bytes: usize,
}

pub fn parse(bytes: &[u8]) -> ParseResult {
    let mut result = ParseResult {
        valid: Vec::new(),
        invalid: Vec::new(),
        padding_bytes: 0,
    };

    let mut i = 0;

    while i < bytes.len() {
        if bytes[i..].starts_with(&SYNC) {

            // sync marker found - extract header fields
            let src_id   = bytes[i + OFFSET_SRC];
            let dst_id   = bytes[i + OFFSET_DST];
            let data_len = bytes[i + OFFSET_DATA_LEN] as usize;
            let reserved = [bytes[i + OFFSET_RESERVED], bytes[i + OFFSET_RESERVED + 1]];

            // Null packets: no validation at all - scan forward to termination and accept
            if bytes[i + OFFSET_TYPE] == 0x00 {
                let packet = Packet { packet_type: PacketType::Null, src_id, dst_id, reserved, data_len: data_len as u8, data: Vec::new() };
                result.valid.push(packet);
                i += OFFSET_DATA;
                while i < bytes.len() && !bytes[i..].starts_with(&TERMINATION) {
                    i += 1;
                }
                i += TERMINATION.len();
                continue;
            }

            // ****** DATA LENGTH VALIDATION ******
            if data_len > MAX_DATA_LEN
                || !bytes[i + OFFSET_DATA + data_len + CHECKSUM_LEN..].starts_with(&TERMINATION)
            {
                result.invalid.push((InvalidReason::BadDataLength, None));
                i += OFFSET_DATA;
                while i < bytes.len() && !bytes[i..].starts_with(&TERMINATION) {
                    i += 1;
                }
                i += TERMINATION.len();
                continue;
            }
            // ****** END DATA LENGTH VALIDATION ******

            // ****** PACKET TYPE VALIDATION ******
            let packet_type = match PacketType::from_byte(bytes[i + OFFSET_TYPE]) {
                Some(t) => t,
                None => {
                    result.invalid.push((InvalidReason::BadPacketType, None));
                    i += MIN_PACKET_LEN + data_len;
                    continue;
                }
            };
            // ****** END PACKET TYPE VALIDATION ******

            // extract data and create packet
            let data = bytes[i + OFFSET_DATA..i + OFFSET_DATA + data_len].to_vec();
            let packet = Packet { packet_type, src_id, dst_id, reserved, data_len: data_len as u8, data };

            // ****** DEVICE ID VALIDATION ******
            let valid_src = packet.src_id == CONTROLLER_ID || OOPS_IDS.contains(&packet.src_id);
            let valid_dst = packet.dst_id == CONTROLLER_ID || OOPS_IDS.contains(&packet.dst_id) || packet.dst_id == BROADCAST_ID;
            if !valid_src || !valid_dst {
                result.invalid.push((InvalidReason::BadDeviceId, Some(packet)));
                i += MIN_PACKET_LEN + data_len;
                continue;
            }
            // ****** END DEVICE ID VALIDATION ******

            // ****** PACKET DIRECTION VALIDATION ******
            let valid_src_dst = if packet.src_id == CONTROLLER_ID {
                OOPS_IDS.contains(&packet.dst_id) || packet.dst_id == BROADCAST_ID
            } else {
                packet.dst_id == CONTROLLER_ID || packet.dst_id == BROADCAST_ID
            };
            let valid_type_src = match packet.packet_type.sender_is_controller() {
                Some(should_be_controller) => (packet.src_id == CONTROLLER_ID) == should_be_controller,
                None => true,
            };
            if !valid_src_dst || !valid_type_src {
                result.invalid.push((InvalidReason::BadPacketDirection, Some(packet)));
                i += MIN_PACKET_LEN + data_len;
                continue;
            }
            // ****** END PACKET DIRECTION VALIDATION ******

            // ****** CHECKSUM VALIDATION ******
            let computed = compute_checksum(&bytes[i + OFFSET_TYPE..i + OFFSET_DATA + data_len]);
            let stored = bytes[i + OFFSET_DATA + data_len] as u16
                       | (bytes[i + OFFSET_DATA + data_len + 1] as u16) << 8;
            if computed != stored {
                result.invalid.push((InvalidReason::BadChecksum, Some(packet)));
                i += MIN_PACKET_LEN + data_len;
                continue;
            }
            // ****** END CHECKSUM VALIDATION ******

            result.valid.push(packet);
            i += MIN_PACKET_LEN + data_len;
        } else {
            // sync marker not found - add the number of padding bytes
            result.padding_bytes += 1;
            i += 1;
        }
    }

    result
}
