mod seek_table;
mod stream_info;
mod picture;
mod vorbis_comment;
pub use vorbis_comment::VorbisComment;
pub use stream_info::StreamInfo;
pub use picture::*;
pub use seek_table::SeekTable;
mod data;
pub type BlockBytes = Block<Vec<u8>>;
use std::ops::{Deref, DerefMut};



use crate::Result;

pub trait BlockType: ConvertBytes {
    const BLOCK_TYPE: u8;
}
impl BlockType for StreamInfo {
    const BLOCK_TYPE: u8 = 0;
}
impl BlockType for SeekTable {
    const BLOCK_TYPE: u8 = 3;
}
impl BlockType for VorbisComment {
    const BLOCK_TYPE: u8 = 4;
}
impl BlockType for Picture {
    const BLOCK_TYPE: u8 = 6;
}


#[derive(Debug, Clone)]
pub struct Block<T> {
    last_metadata_block: bool,
    block_type: u8,
    block_size: u32,
    block_data: T,
}

impl<T: ConvertBytes> Deref for Block<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.block_data
    }
}
impl<T: ConvertBytes> DerefMut for Block<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.block_data
    }
}



impl<T> Block<T> {

    pub const STREAMINFO: u8 = 0;
    pub const PADDING: u8 = 1;
    pub const APPLICATION: u8 = 2;
    pub const SEEKTABLE: u8 = 3;
    pub const VORBIS_COMMENT: u8 = 4;
    pub const CUESHEET: u8 = 5;
    pub const PICTURE: u8 = 6;
    pub fn new(last_block: bool, ty: u8, size: u32, data: T) -> Block<T> {
        Self {
            last_metadata_block: last_block,
            block_type: ty,
            block_size: size,
            block_data: data,
        }
    }
    pub fn last_metadata_block(&self) -> bool {
        self.last_metadata_block
    }
    pub fn block_data(&self) -> &T {
        &self.block_data
    }

    pub fn block_type_str(&self) -> &'static str {
        match self.block_type {
            0 => "STREAMINFO",
            1 => "PADDING",
            2 => "APPLICATION",
            3 => "SEEKTABLE",
            4 => "VORBIS_COMMENT",
            5 => "CUESHEET",
            6 => "PICTURE",
            7..=126 => "reserved",
            _ => "",
        }
    }
    pub fn is<B: BlockType>(&self) -> bool {
        self.block_type == B::BLOCK_TYPE
    }
    pub fn block_type(&self) -> u8 {
        self.block_type
    }
    pub fn block_size(&self) -> u32 {
        self.block_size
    }
    pub fn into_inner(self) -> T {
        self.block_data
    }
}

pub trait ConvertBytes: Sized {
    fn from_bytes(buf: Vec<u8>) -> Result<Self>;
    fn into_bytes(self) -> Vec<u8>;
}

impl Block<Vec<u8>> {
    pub fn convert<T: ConvertBytes>(self) -> Result<Block<T>> {
        Ok(Block {
            last_metadata_block: self.last_metadata_block,
            block_type: self.block_type,
            block_size: self.block_size,
            block_data: T::from_bytes(self.block_data)?,
        })
    }
}


impl<T: ConvertBytes> Block<T>  {
    pub fn to_bytes(mut self) -> Vec<u8> {
        let inner_bytes = self.block_data.into_bytes();
        self.block_size = inner_bytes.len() as u32;
        let mut buf = vec![0; 4 + inner_bytes.len()];
        buf[0] = self.block_type + (self.last_metadata_block as u8) * 128;
        buf[1..4].copy_from_slice(&self.block_size.to_be_bytes()[1..]);
        buf[4..].copy_from_slice(&inner_bytes);
        buf
    }
}