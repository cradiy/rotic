
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleRate {
    /// Get from STREAMINFO metadata block
    FromMetaBlock,
    Hz88_2k,
    Hz176_4k,
    Hz192k,
    Hz8k,
    Hz16k,
    Hz22_05k,
    Hz24k,
    Hz32k,
    Hz44_1k,
    Hz48k,
    Hz96k,
    /// Get 8 bit sample rate (in kHz) from end of header
    KHz8b,
    /// Get 16 bit sample rate (in Hz) from end of header
    Hz16b,
    /// Get 16 bit sample rate (in tens of Hz) from end of header
    Hz16bTens,
    /// Invalid, to prevent sync-fooling string of 1s
    Invalid
}
impl From<u8> for SampleRate {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}
impl From<SampleRate> for u8 {
    fn from(value: SampleRate) -> Self {
        value.to_u8()
    }
}
impl SampleRate {
    pub fn from_u8(val: u8) -> Self {
        match val {
            0b0000 => SampleRate::FromMetaBlock,
            0b0001 => SampleRate::Hz88_2k,
            0b0010 => SampleRate::Hz176_4k,
            0b0011 => SampleRate::Hz192k,
            0b0100 => SampleRate::Hz8k,
            0b0101 => SampleRate::Hz16k,
            0b0110 => SampleRate::Hz22_05k,
            0b0111 => SampleRate::Hz24k,
            0b1000 => SampleRate::Hz32k,
            0b1001 => SampleRate::Hz44_1k,
            0b1010 => SampleRate::Hz48k,
            0b1011 => SampleRate::Hz96k,
            0b1100 => SampleRate::KHz8b,
            0b1101 => SampleRate::Hz16b,
            0b1110 => SampleRate::Hz16bTens,
            0b1111 => SampleRate::Invalid,
            _ => unreachable!("Invalid sample rate")
        }
    }
    pub fn to_u8(&self) -> u8 {
        match self {
            SampleRate::FromMetaBlock => 0b0000,
            SampleRate::Hz88_2k => 0b0001,
            SampleRate::Hz176_4k => 0b0010,
            SampleRate::Hz192k => 0b0011,
            SampleRate::Hz8k => 0b0100,
            SampleRate::Hz16k => 0b0101,
            SampleRate::Hz22_05k => 0b0110,
            SampleRate::Hz24k => 0b0111,
            SampleRate::Hz32k => 0b1000,
            SampleRate::Hz44_1k => 0b1001,
            SampleRate::Hz48k => 0b1010,
            SampleRate::Hz96k => 0b1011,
            SampleRate::KHz8b => 0b1100,
            SampleRate::Hz16b => 0b1101,
            SampleRate::Hz16bTens => 0b1110,
            SampleRate::Invalid => 0b1111,
        }
    }
}



pub struct FrameHeader {
    sync_code: [u8; 2],
    reserved: bool,
    blocking_strategy: bool,
    block_size: u8,
    sample_rate: SampleRate,
}
