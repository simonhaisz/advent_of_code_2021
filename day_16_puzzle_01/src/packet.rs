#[derive(Debug, PartialEq)]
pub enum Packet {
    Literal(Box<LiteralPacket>),
    Operator(Box<OperatorPacket>),
}

#[derive(Debug, PartialEq)]
pub struct LiteralPacket {
    version: u32,
    value: u32,
}

#[derive(Debug, PartialEq)]
pub struct OperatorPacket {
    version: u32,
    sub_packets: Vec<Packet>,
}

impl LiteralPacket {
    pub fn new(version: u32, value: u32) -> LiteralPacket {
        LiteralPacket {
            version,
            value,
        }
    }
}

impl OperatorPacket {
    pub fn new(version: u32, sub_packets: Vec<Packet>) -> OperatorPacket {
        OperatorPacket {
            version,
            sub_packets,
        }
    }
}