use super::ConvertBytes;
use crate::const_array;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct StreamInfo {
    pub min_block_size: u16,
    pub max_block_size: u16,
    pub min_frame_size: u32,
    pub max_frame_size: u32,
    pub sample_rate: u32,
    pub channels: u8,
    pub bps: u8,
    pub total_samples: u64,
    pub md5: [u8; 16],
}
macro_rules! compute {
    ($v: expr, $as:ident, $offset:expr) => {
        (($v as $as) << $offset)
    };
}
impl ConvertBytes for StreamInfo {
    fn from_bytes(buf: Vec<u8>) -> crate::Result<Self> {
        if buf.len() != 34 {
            return Err(crate::error::Error::InvalidFormat);
        }
        let min_block_size = u16::from_be_bytes(const_array!(buf, 0, 2));
        let max_block_size = u16::from_be_bytes(const_array!(buf, 2, 2));
        let frame_arr = &buf[4..10];
        let min_frame_size = u32::from_be_bytes(const_array!(@start(0), frame_arr => 0, 1, 2));
        let max_frame_size = u32::from_be_bytes(const_array!(@start(0), frame_arr => 3, 4, 5));
        let sample_rate =
            ((buf[10] as u32) << 12) + ((buf[11] as u32) << 4) + ((buf[12] as u32) >> 4);
        let channels = ((buf[12] >> 1) & 0b0111) + 1;
        let bps = ((buf[12] & 0x1) << 4) + (buf[13] >> 4) + 1;
        let total_samples = (((buf[13] & 0x0f) as u64) << 32)
            + compute!(buf[14], u64, 24)
            + compute!(buf[15], u64, 16)
            + compute!(buf[16], u64, 8)
            + buf[17] as u64;
        let md5 = const_array!(buf, 18, 16);
        Ok(Self {
            min_block_size,
            max_block_size,
            min_frame_size,
            max_frame_size,
            sample_rate,
            channels,
            bps,
            total_samples,
            md5,
        })
    }

    fn into_bytes(self) -> Vec<u8> {
        let mut bytes = vec![0; 34];
        bytes[0..2].copy_from_slice(&self.min_block_size.to_be_bytes());
        bytes[2..4].copy_from_slice(&self.min_block_size.to_be_bytes());
        bytes[4..7].copy_from_slice(&self.min_frame_size.to_be_bytes()[1..]);
        bytes[7..10].copy_from_slice(&self.max_frame_size.to_be_bytes()[1..]);
        bytes[10] = (self.sample_rate >> 12) as u8;
        bytes[11] = ((self.sample_rate >> 4) & 0xff) as u8;
        bytes[12] = ((self.sample_rate << 4) & 0xff) as u8;
        bytes[12] |= ((self.channels - 1) << 1) & 0xff;
        bytes[12] |= ((self.bps - 1) >> 4) & 0xff;
        bytes[13] = ((self.bps - 1) << 4) & 0xff;
        bytes[13] |= (self.total_samples >> 32) as u8;
        bytes[14..18].copy_from_slice(&self.total_samples.to_be_bytes()[4..]);
        bytes[18..34].copy_from_slice(&self.md5);
        bytes
    }
}
