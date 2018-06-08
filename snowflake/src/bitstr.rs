use bit::Bit;
use std::str::Bytes;

pub struct BitStr<'a> {
    bytes: Bytes<'a>,
    cur: Option<u8>,
    cur_idx: u8
}

impl<'a> BitStr<'a> {
    pub fn from_str(s: &'a str) -> BitStr {
        BitStr {
            bytes: s.bytes(),
            cur: None,
            cur_idx: 0
        }
    }
}

impl<'a> Iterator for BitStr<'a> {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.is_none() || self.cur_idx >= 8 {
            self.cur = self.bytes.next();
            self.cur_idx = 0;
        }

        if let Some(byte) = self.cur {
            let bit = Bit::from_u8((byte << self.cur_idx) & 128);
            self.cur_idx += 1;
            return Some(bit);
        }

        None
    }
}