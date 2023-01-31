/// Allows conversion of an address into a vector of big-endian bytes
pub trait Address {
    /// Returns the bytes of the address value in big-endian
    fn to_be_bytes(&self) -> Vec<u8>;
}

/// 16-bit address
#[derive(Debug, Copy, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Address16(pub u16);

impl Address for Address16 {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

impl From<Address16> for u32 {
    fn from(addr: Address16) -> u32 {
        addr.0 as u32
    }
}

impl From<&Address16> for u32 {
    fn from(addr: &Address16) -> u32 {
        addr.0 as u32
    }
}

// TODO: Restrict the value to 24 bits
/// 24-bit address
#[derive(Debug, Copy, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Address24(pub u32);

impl Address for Address24 {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes()[1..].to_vec()
    }
}

impl From<Address24> for u32 {
    fn from(addr: Address24) -> u32 {
        addr.0
    }
}

impl From<&Address24> for u32 {
    fn from(addr: &Address24) -> u32 {
        addr.0
    }
}

/// 32-bit address
#[derive(Debug, Copy, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Address32(pub u32);

impl Address for Address32 {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

impl From<Address32> for u32 {
    fn from(addr: Address32) -> u32 {
        addr.0
    }
}

impl From<&Address32> for u32 {
    fn from(addr: &Address32) -> u32 {
        addr.0
    }
}

/// 16-bit data record count
#[derive(Debug, Copy, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Count16(pub u16);

impl From<Count16> for u32 {
    fn from(count: Count16) -> u32 {
        count.0 as u32
    }
}

impl From<&Count16> for u32 {
    fn from(count: &Count16) -> u32 {
        count.0 as u32
    }
}

// TODO: Restrict the value to 24 bits
/// 24-bit data record count
#[derive(Debug, Copy, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Count24(pub u32);

impl From<Count24> for u32 {
    fn from(count: Count24) -> u32 {
        count.0
    }
}

impl From<&Count24> for u32 {
    fn from(count: &Count24) -> u32 {
        count.0
    }
}

/// Record data field
#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Data<T> {
    /// Start address
    pub address: T,
    /// Data bytes
    pub data: Vec<u8>,
}

/// An SRecord
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/SREC_(file_format)#Record_types)
/// for specific record usage information.
#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum Record {
    /// Header
    S0(String),
    /// Data with 16-bit address
    S1(Data<Address16>),
    /// Data with 24-bit address
    S2(Data<Address24>),
    /// Data with 32-bit address
    S3(Data<Address32>),
    // S4 - reserved
    /// 16-bit data record count
    S5(Count16),
    /// 24-bit data record count
    S6(Count24),
    /// 32-bit start address
    S7(Address32),
    /// 24-bit start address
    S8(Address24),
    /// 16-bit start address
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
    fn address16_ref_to_be_bytes() {
        let a = &Address16(0x1234);

        let b = a.to_be_bytes();

        assert_eq!(b, [0x12, 0x34]);
    }

    #[test]
    fn address16_into_u32() {
        let a = Address16(0x1234);

        let b: u32 = a.into();

        assert_eq!(b, 0x1234u32);
    }

    #[test]
    fn address16_ref_into_u32() {
        let a = &Address16(0x1234);

        let b: u32 = a.into();

        assert_eq!(b, 0x1234u32);
    }

    #[test]
    fn address24_to_be_bytes() {
        let a = Address24(0x123456);

        let b = a.to_be_bytes();

        assert_eq!(b, [0x12, 0x34, 0x56]);
    }

    #[test]
    fn address24_ref_to_be_bytes() {
        let a = &Address24(0x123456);

        let b = a.to_be_bytes();

        assert_eq!(b, [0x12, 0x34, 0x56]);
    }

    #[test]
    fn address24_into_u32() {
        let a = Address24(0x123456);

        let b: u32 = a.into();

        assert_eq!(b, 0x123456u32);
    }

    #[test]
    fn address24_ref_into_u32() {
        let a = &Address24(0x123456);

        let b: u32 = a.into();

        assert_eq!(b, 0x123456u32);
    }

    #[test]
    fn address32_to_be_bytes() {
        let a = Address32(0x12345678);

        let b = a.to_be_bytes();

        assert_eq!(b, [0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn address32_ref_to_be_bytes() {
        let a = &Address32(0x12345678);

        let b = a.to_be_bytes();

        assert_eq!(b, [0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn address32_into_u32() {
        let a = Address32(0x12345678);

        let b: u32 = a.into();

        assert_eq!(b, 0x12345678u32);
    }

    #[test]
    fn address32_ref_into_u32() {
        let a = &Address32(0x12345678);

        let b: u32 = a.into();

        assert_eq!(b, 0x12345678u32);
    }

    #[test]
    fn count16_into_u32() {
        let a = Count16(0x1234);

        let b: u32 = a.into();

        assert_eq!(b, 0x1234);
    }

    #[test]
    fn count16_ref_into_u32() {
        let a = &Count16(0x1234);

        let b: u32 = a.into();

        assert_eq!(b, 0x1234);
    }

    #[test]
    fn count24_into_u32() {
        let a = Count24(0x123456);

        let b: u32 = a.into();

        assert_eq!(b, 0x123456);
    }

    #[test]
    fn count24_ref_into_u32() {
        let a = &Count24(0x123456);

        let b: u32 = a.into();

        assert_eq!(b, 0x123456);
    }
}
