// The possible types for a packet
pub enum PacketType {
    Null,
    Request,
    Telemetry,
    InboundData,
    OutboundData,
    Command,
    AckNack,
}

pub enum InvalidReason {
    BadPacketType,    // unrecognised type (not 00-06)
    BadDataLength,    // data_len exceeds buffer bounds
    BadChecksum,      // checksum does not match
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
