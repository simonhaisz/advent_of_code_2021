#[derive(Debug, PartialEq)]
pub enum Packet {
    Literal(Box<LiteralPacket>),
    Operator(Box<OperatorPacket>),
}

impl Packet {
    pub fn version_total(&self) -> u64 {
        match self {
            Packet::Literal(d) => d.version(),
            Packet::Operator(d) => {
                let mut sub_total = d.version();
                for sub in d.sub_packets().iter() {
                    sub_total += sub.version_total();
                }
                sub_total
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LiteralPacket {
    version: u64,
    value: u64,
}

#[derive(Debug, PartialEq)]
pub struct OperatorPacket {
    version: u64,
    sub_packets: Vec<Packet>,
}

impl LiteralPacket {
    pub fn new(version: u64, value: u64) -> LiteralPacket {
        LiteralPacket {
            version,
            value,
        }
    }

    pub fn version(&self) -> u64 {
        self.version
    }
}

impl OperatorPacket {
    pub fn new(version: u64, sub_packets: Vec<Packet>) -> OperatorPacket {
        OperatorPacket {
            version,
            sub_packets,
        }
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn sub_packets(&self) -> &Vec<Packet> {
        &self.sub_packets
    }
}