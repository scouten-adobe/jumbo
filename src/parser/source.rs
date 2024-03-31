use std::fmt::Debug;

pub trait Source: Debug + Sized {
    type Error: Debug;

    fn read_bytes(&self, data: &mut [u8]) -> Result<Self, Self::Error>;
    fn as_bytes(&self) -> Result<Vec<u8>, Self::Error>;
    fn len(&self) -> usize;

    fn split_at(&self, len: usize) -> Result<(Self, Self), Self::Error>;
    fn offset_of_subsource(&self, subsource: Self) -> Option<usize>;

    fn read_be32(&self) -> Result<(u32, Self), Self::Error> {
        let mut b = [0u8; 4];
        let remainder = self.read_bytes(&mut b)?;

        let mut res = 0u32;
        for byte in b {
            res = (res << 8) + byte as u32;
        }

        Ok((res, remainder))
    }

    fn read_be64(&self) -> Result<(u64, Self), Self::Error> {
        let mut b = [0u8; 8];
        let remainder = self.read_bytes(&mut b)?;

        let mut res = 0u64;
        for byte in b {
            res = (res << 8) + byte as u64;
        }

        Ok((res, remainder))
    }
}

/// Returned when trying to read past the end of a slice.
#[derive(Debug)]
pub struct UnexpectedEof;

impl Source for &[u8] {
    type Error = UnexpectedEof;

    fn read_bytes(&self, data: &mut [u8]) -> Result<Self, Self::Error> {
        if data.len() > self.len() {
            return Err(UnexpectedEof);
        }

        let self_as_u8: &[u8] = self;
        let (wanted, remainder) = self_as_u8.split_at(data.len());
        data.copy_from_slice(wanted);
        Ok(remainder)
    }

    fn as_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        // Replace with COW
        Ok(self.to_vec())
    }

    fn len(&self) -> usize {
        let self_as_u8: &[u8] = self;
        self_as_u8.len()
    }

    fn split_at(&self, len: usize) -> Result<(Self, Self), Self::Error> {
        if len > self.len() {
            return Err(UnexpectedEof);
        }

        let self_as_u8: &[u8] = self;
        let (wanted, remainder) = self_as_u8.split_at(len);
        Ok((wanted, remainder))
    }

    fn offset_of_subsource(&self, _subsource: Self) -> Option<usize> {
        unimplemented!();
    }
}
