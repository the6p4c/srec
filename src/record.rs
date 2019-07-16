pub trait Address {
    fn to_be_bytes(&self) -> Vec<u8>;
}

#[derive(Debug, PartialEq)]
pub struct Address16(pub u16);

impl Address for Address16 {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

// TODO: Restrict the value to 24 bits
#[derive(Debug, PartialEq)]
pub struct Address24(pub u32);

impl Address for Address24 {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes()[1..].to_vec()
    }
}

#[derive(Debug, PartialEq)]
pub struct Address32(pub u32);

impl Address for Address32 {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

#[derive(Debug, PartialEq)]
pub struct Count16(pub u16);

// TODO: Restrict the value to 24 bits
#[derive(Debug, PartialEq)]
pub struct Count24(pub u32);

#[derive(Debug, PartialEq)]
pub struct Data<T> {
    pub address: T,
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum Record {
    S0(String),
    S1(Data<Address16>),
    S2(Data<Address24>),
    S3(Data<Address32>),
    // S4 - reserved
    S5(Count16),
    S6(Count24),
    S7(Address32),
    S8(Address24),
    S9(Address16),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address16_to_be_bytes() {
        let a = Address16(0x1234);

        let b = a.to_be_bytes();

        assert_eq!(b, [0x12, 0x34]);
    }

    #[test]
    fn address24_to_be_bytes() {
        let a = Address24(0x123456);

        let b = a.to_be_bytes();

        assert_eq!(b, [0x12, 0x34, 0x56]);
    }

    #[test]
    fn address32_to_be_bytes() {
        let a = Address32(0x12345678);

        let b = a.to_be_bytes();

        assert_eq!(b, [0x12, 0x34, 0x56, 0x78]);
    }
}
