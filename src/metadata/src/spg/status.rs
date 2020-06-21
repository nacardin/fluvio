use std::io::{Error, ErrorKind};

use kf_protocol::{Decoder, Encoder};
use kf_protocol::Version;
use kf_protocol::derive::*;
use kf_protocol::bytes::{Buf, BufMut};


#[derive(Encode, Decode, Default, Debug)]
pub struct SpuGroupStatus {
    /// Status resolution
    pub resolution: SpuGroupStatusResolution,

    /// Reason for Status resolution (if applies)
    pub reason: Option<String>,
}


#[derive(Debug)]
pub enum SpuGroupStatusResolution {
    Init,
    Invalid,
    Reserved,
}

// -----------------------------------
// Implementation - FlvSpuGroupResolution
// -----------------------------------
impl Default for SpuGroupStatusResolution {
    fn default() -> Self {
        Self::Init
    }
}

impl Encoder for SpuGroupStatusResolution {
   
    fn write_size(&self, version: Version) -> usize {
        (0 as u8).write_size(version)
    }

    // encode match
    fn encode<T>(&self, dest: &mut T, version: Version) -> Result<(), Error>
    where
        T: BufMut,
    {
        // ensure buffer is large enough
        if dest.remaining_mut() < self.write_size(version) {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                format!(
                    "not enough capacity for group spu resolution {}",
                    self.write_size(version)
                ),
            ));
        }

        match self {
            Self::Init => {
                let typ: u8 = 0;
                typ.encode(dest, version)?;
            }
            Self::Invalid => {
                let typ: u8 = 1;
                typ.encode(dest, version)?;
            }
            Self::Reserved => {
                let typ: u8 = 2;
                typ.encode(dest, version)?;
            }
        }

        Ok(())
    }
}

impl Decoder for SpuGroupStatusResolution {
    fn decode<T>(&mut self, src: &mut T, version: Version) -> Result<(), Error>
    where
        T: Buf,
    {
        let mut value: u8 = 0;
        value.decode(src, version)?;
        match value {
            0 => *self = Self::Init,
            1 => *self = Self::Invalid,
            2 => *self = Self::Reserved,
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedEof,
                    format!("invalid value for group spu resolution: {}", value),
                ))
            }
        }

        Ok(())
    }
}
