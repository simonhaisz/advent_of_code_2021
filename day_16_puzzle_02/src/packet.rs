#[derive(Debug, PartialEq)]
pub enum Packet {
    Sum(Box<OperatorPacket>),
    Product(Box<OperatorPacket>),
    Min(Box<OperatorPacket>),
    Max(Box<OperatorPacket>),
    Literal(Box<LiteralPacket>),
    GreaterThan(Box<OperatorPacket>),
    LessThan(Box<OperatorPacket>),
    Equalsto(Box<OperatorPacket>),
}

impl Packet {
    pub fn value(&self) -> u64 {
        match self {
            Packet::Sum(p) => p.sum(),
            Packet::Product(p) => p.product(),
            Packet::Min(p) => p.min(),
            Packet::Max(p) => p.max(),
            Packet::Literal(p) => p.value(),
            Packet::GreaterThan(p) => p.greater_than(),
            Packet::LessThan(p) => p.less_than(),
            Packet::Equalsto(p) => p.equals_to(),
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

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl OperatorPacket {
    pub fn new(version: u64, sub_packets: Vec<Packet>) -> OperatorPacket {
        OperatorPacket {
            version,
            sub_packets,
        }
    }

    pub fn sum(&self) -> u64 {
        self.sub_packets
            .iter()
            .map(|s| s.value())
            .sum()
    }
    
    pub fn product(&self) -> u64 {
        self.sub_packets
            .iter()
            .map(|s| s.value())
            .product()
    }

    pub fn min(&self) -> u64 {
        self.sub_packets
            .iter()
            .map(|s| s.value())
            .min().expect("There should be at least one sub-packet to determine minimum")
    }

    pub fn max(&self) -> u64 {
        self.sub_packets
            .iter()
            .map(|s| s.value())
            .max().expect("There should be at least one sub-packet to determine maxium")
    }

    pub fn greater_than(&self) -> u64 {
        if self.sub_packets.len() != 2 {
            panic!("There should be exactly two sub-packets to determine greater-than")
        }

        if self.sub_packets[0].value() > self.sub_packets[1].value() {
            1
        } else {
            0
        }
    }

    pub fn less_than(&self) -> u64 {
        if self.sub_packets.len() != 2 {
            panic!("There should be exactly two sub-packets to determine less-than")
        }

        if self.sub_packets[0].value() < self.sub_packets[1].value() {
            1
        } else {
            0
        }
    }

    pub fn equals_to(&self) -> u64 {
        if self.sub_packets.len() != 2 {
            panic!("There should be exactly two sub-packets to determine equals-to")
        }

        if self.sub_packets[0].value() == self.sub_packets[1].value() {
            1
        } else {
            0
        }
    }
}