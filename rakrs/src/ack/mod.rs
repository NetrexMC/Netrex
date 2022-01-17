pub mod queue;
use binary_utils::error::BinaryError;
use binary_utils::*;
use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub enum Record {
    Single(SingleRecord),
    Range(RangeRecord),
}

impl Record {
    pub fn is_single(&self) -> bool {
        match *self {
            Self::Range(_) => false,
            Self::Single(_) => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SingleRecord {
    pub sequence: u32,
}

#[derive(Debug, Clone)]
pub struct RangeRecord {
    pub start: u32,
    pub end: u32,
}

impl RangeRecord {
    /// Gets the sequences in the range of [start, end).
    pub fn get_sequences(&self) -> Vec<u32> {
        let mut seqs = vec![];
        let highest = if self.end > self.start {
            self.end
        } else {
            self.start
        };
        let lowest = if self.end > self.start {
            self.start
        } else {
            self.end
        };
        for i in lowest..highest {
            seqs.push(i);
        }
        seqs
    }
}

#[derive(Debug, Clone)]
pub struct Ack {
    pub count: u16,
    pub records: Vec<Record>,
    pub id: AckIds,
}

impl Ack {
    pub fn new(count: u16, nack: bool) -> Self {
        Self {
            count,
            records: Vec::new(),
            id: match nack {
                true => AckIds::Acknowledge,
                false => AckIds::NoAcknowledge,
            },
        }
    }
}

impl Streamable for Ack {
    fn compose(source: &[u8], _position: &mut usize) -> Result<Self, BinaryError> {
        let mut stream = Cursor::new(source);
        let id = stream.read_u8().unwrap();
        let count = stream.read_u16::<BE>().unwrap();
        let mut records: Vec<Record> = Vec::new();
        for _ in 0..count {
            if stream.read_u8().unwrap() == 1 {
                let record: SingleRecord = SingleRecord {
                    sequence: stream.read_u24::<BE>().unwrap(),
                };

                records.push(Record::Single(record));
            } else {
                let record: RangeRecord = RangeRecord {
                    start: stream.read_u24::<BE>().unwrap(),
                    end: stream.read_u24::<BE>().unwrap(),
                };

                records.push(Record::Range(record));
            }
        }

        Ok(Self {
            count,
            records,
            id: AckIds::from_byte(id),
        })
    }

    fn parse(&self) -> Result<Vec<u8>, BinaryError> {
        let mut stream = Vec::<u8>::new();
        stream.write_u8(AckIds::Acknowledge as u8)?;
        stream.write_u16::<BE>(self.count)?;

        for record in self.records.iter() {
            match record {
                Record::Single(rec) => {
                    stream.write_u8(1)?;
                    stream.write_u24::<BE>(rec.sequence)?;
                }
                Record::Range(rec) => {
                    stream.write_u8(0).unwrap();
                    stream.write_u24::<BE>(rec.start)?;
                    stream.write_u24::<BE>(rec.end)?;
                }
            }
        }
        Ok(stream)
    }
}

#[derive(Debug, Clone)]
pub enum AckIds {
    Acknowledge = 0xc0,
    NoAcknowledge = 0xa0,
}

impl AckIds {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0xa0 => Self::NoAcknowledge,
            _ => Self::Acknowledge,
        }
    }

    pub fn to_byte(&self) -> u8 {
        match *self {
            Self::Acknowledge => 0xc0,
            Self::NoAcknowledge => 0xa0,
        }
    }
}

pub fn is_ack_or_nack(byte: u8) -> bool {
    byte == 0xa0 || byte == 0xc0
}
