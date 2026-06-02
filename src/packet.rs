// Constants, data structures and method for OOPS packets

// Protocol constants
pub const SYNC: [u8; 3]        = [0xA5, 0xF0, 0xEE];
pub const TERMINATION: [u8; 3] = [0xEE, 0x0F, 0x5A];
pub const MAX_DATA_LEN: usize  = 241;
pub const CHECKSUM_LEN: usize  = 2;
pub const MIN_PACKET_LEN: usize = 14; // 9 header + 2 checksum + 3 termination

// Valid device IDs as defined in the datasheet
pub const CONTROLLER_ID: u8 = 1;
pub const OOPS_IDS: [u8; 4] = [4, 5, 6, 7];
pub const BROADCAST_ID: u8  = 0xFF;

// Byte offsets from the start of the sync marker
pub const OFFSET_TYPE:     usize = 3;
pub const OFFSET_SRC:      usize = 4;
pub const OFFSET_DST:      usize = 5;
pub const OFFSET_DATA_LEN: usize = 6;
pub const OFFSET_RESERVED: usize = 7;
pub const OFFSET_DATA:     usize = 9;

pub enum PacketType {
    Null,
    Request,
    Telemetry,
    InboundData,
    OutboundData,
    Command,
    AckNack,
}

impl PacketType {

    pub fn to_byte(&self) -> u8 {
        match self {
            PacketType::Null         => 0x00,
            PacketType::Request      => 0x01,
            PacketType::Telemetry    => 0x02,
            PacketType::InboundData  => 0x03,
            PacketType::OutboundData => 0x04,
            PacketType::Command      => 0x05,
            PacketType::AckNack      => 0x06,
        }
    }

    // Returns None for unrecognised type bytes (not 00-06).
    pub fn from_byte(b: u8) -> Option<PacketType> {
        match b {
            0x00 => Some(PacketType::Null),
            0x01 => Some(PacketType::Request),
            0x02 => Some(PacketType::Telemetry),
            0x03 => Some(PacketType::InboundData),
            0x04 => Some(PacketType::OutboundData),
            0x05 => Some(PacketType::Command),
            0x06 => Some(PacketType::AckNack),
            _    => None,
        }
    }

    // Some(true)  → must be sent by the controller (ID 1)
    // Some(false) → must be sent by an OOPS system (ID 4–7)
    // None        → direction undefined (Null)
    pub fn sender_is_controller(&self) -> Option<bool> {
        match self {
            PacketType::Request | PacketType::OutboundData | PacketType::Command => Some(true),
            PacketType::Telemetry | PacketType::InboundData | PacketType::AckNack => Some(false),
            _ => None,
        }
    }
}

pub enum InvalidReason {
    BadPacketType,      // unrecognised type (not 00-06)
    BadDataLength,      // data_len > 241, or termination bytes not where expected
    BadDeviceId,        // src or dst is not a valid device ID
    BadPacketDirection, // incoherent src/dst combination, or type/src mismatch
    BadChecksum,        // checksum does not match
}

// The general structure for a single packet
pub struct Packet {
    pub packet_type: PacketType,
    pub src_id: u8,
    pub dst_id: u8,
    pub reserved: [u8; 2],
    pub data_len: u8,
    pub data: Vec<u8>,
}

impl Packet {
    pub fn checksum(&self) -> u16 {
        let mut bytes = vec![
            self.packet_type.to_byte(),
            self.src_id,
            self.dst_id,
            self.data_len,
            self.reserved[0],
            self.reserved[1],
        ];
        bytes.extend_from_slice(&self.data);
        compute_checksum(&bytes)
    }
}

fn compute_checksum(bytes: &[u8]) -> u16 {
    let mut acc: u16 = 0xACE1;
    for &byte in bytes {
        acc = acc.rotate_left(3);
        acc = acc ^ byte as u16;
        acc = acc.wrapping_add(0x1F);
    }
    acc
}
