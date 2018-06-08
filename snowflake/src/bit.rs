#[derive(Debug)]
pub struct Bit(Option<()>);

impl Bit {
    pub fn high() -> Bit {
        Bit(Some(()))
    }

    pub fn low() -> Bit {
        Bit(None)
    }

    pub fn from_u8(n: u8) -> Bit {
        match n {
            0 => Bit::low(),
            _ => Bit::high()
        }
    }

    pub fn as_bool(&self) -> bool {
        self.0.is_some()
    }
}

impl PartialEq for Bit {
    fn eq(&self, other: &Bit) -> bool {
        self.as_bool() == other.as_bool()
    }
}

impl Eq for Bit {}