#[derive(Clone, Debug)]
pub struct Reliability {
    flag: ReliabilityFlag,
}

impl Reliability {
    pub fn new(flag: ReliabilityFlag) -> Self {
        Self { flag }
    }

    pub fn from_bit(byte: u8) -> Self {
        match (byte & 224) >> 5 {
            0 => Self::new(ReliabilityFlag::Unreliable),
            1 => Self::new(ReliabilityFlag::UnreliableSeq),
            2 => Self::new(ReliabilityFlag::Reliable),
            3 => Self::new(ReliabilityFlag::ReliableOrd),
            4 => Self::new(ReliabilityFlag::ReliableSeq),
            5 => Self::new(ReliabilityFlag::UnreliableAck),
            6 => Self::new(ReliabilityFlag::ReliableAck),
            7 => Self::new(ReliabilityFlag::ReliableOrdAck),
            _ => Self::new(ReliabilityFlag::Unreliable),
        }
    }

    pub fn to_byte(&self) -> u8 {
        match self.flag {
            ReliabilityFlag::Unreliable => 0,
            ReliabilityFlag::UnreliableSeq => 1,
            ReliabilityFlag::Reliable => 2,
            ReliabilityFlag::ReliableOrd => 3,
            ReliabilityFlag::ReliableSeq => 4,
            ReliabilityFlag::UnreliableAck => 5,
            ReliabilityFlag::ReliableAck => 6,
            ReliabilityFlag::ReliableOrdAck => 7,
        }
    }

    pub fn is_reliable(byte: u8) -> bool {
        match byte {
            2 | 3 | 4 => true,
            6 | 7 => true,
            _ => false,
        }
    }

    pub fn is_seq(byte: u8) -> bool {
        match byte {
            1 | 4 => true,
            _ => false,
        }
    }

    pub fn is_ord(byte: u8) -> bool {
        if Self::is_seq(byte) {
            return true;
        }
        match byte {
            3 | 7 => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ReliabilityFlag {
    Unreliable,
    UnreliableSeq,
    Reliable,
    ReliableOrd,
    ReliableSeq,
    UnreliableAck,
    ReliableAck,
    ReliableOrdAck,
}

impl ReliabilityFlag {}
