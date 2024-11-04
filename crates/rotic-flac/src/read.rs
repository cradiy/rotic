use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::const_array;
use crate::error::Error::*;
use crate::metadata::{Block, BlockBytes, BlockType};
use crate::Result;

pub fn read_from_stream<R: Read>(buf: &mut R) -> Result<Vec<BlockBytes>> {
    let mut blocks = Vec::new();
    let mut four = [0; 4];
    buf.read_exact(&mut four)?;
    if &four != b"fLaC" {
        return Err(InvalidFormat);
    }

    loop {
        buf.read_exact(&mut four)?;
        let end = (four[0] & 0x80) == 128;
        let ty = four[0] & 0x7F;
        let size = u32::from_be_bytes(const_array!(@start(0), four => 1, 2, 3));
        let mut block_buf = Vec::new();
        buf.take(size as u64).read_to_end(&mut block_buf)?;
        if block_buf.len() != size as usize {
            return Err(InvalidFormat);
        }
        blocks.push(Block::new(end, ty, size, block_buf));
        if end {
            break;
        }
    }

    Ok(blocks)
}

pub fn read_from_path<R: Read>(path: impl AsRef<Path>) -> Result<Vec<BlockBytes>> {
    read_from_stream(&mut File::open(path)?)
}

pub fn read_from_bytes(buf: &[u8]) -> Result<Vec<BlockBytes>> {
    let mut blocks = Vec::new();
    let mut stream = Stream::new(buf);
    if stream.take(4)? != b"fLaC" {
        return Err(InvalidFormat);
    }
    loop {
        let four = stream.take(4)?;
        let end = (four[0] & 0x80) == 128;
        let ty = four[0] & 0x7F;
        let size = u32::from_be_bytes(const_array!(@start(0), four => 1, 2, 3));
        let block_buf = stream.take(size as usize)?;
        blocks.push(Block::new(end, ty, size, block_buf.to_owned()));
        if end {
            break;
        }
    }

    Ok(blocks)
}

pub fn find_meta<B: BlockType, R: Read>(buf: &mut R) -> Result<Option<B>> {
    let mut four = [0; 4];
    buf.read_exact(&mut four)?;
    if &four != b"fLaC" {
        return Err(InvalidFormat);
    }

    loop {
        buf.read_exact(&mut four)?;
        let end = (four[0] & 0x80) == 128;
        let ty = four[0] & 0x7F;
        let size = u32::from_be_bytes(const_array!(@start(0), four => 1, 2, 3));
        let mut block_buf = Vec::new();
        buf.take(size as u64).read_to_end(&mut block_buf)?;
        if block_buf.len() != size as usize {
            return Err(InvalidFormat);
        }
        if B::BLOCK_TYPE == ty {
            return Ok(Some(B::from_bytes(block_buf)?));
        }
        if end {
            break Ok(None);
        }
    }
}

pub fn find_meta_from_bytes<B: BlockType>(bytes: &[u8]) -> Result<Option<B>> {
    let mut stream = Stream::new(bytes);
    if stream.take(4)? != b"fLaC" {
        return Err(InvalidFormat);
    }

    loop {
        let four = stream.take(4)?;
        let end = (four[0] & 0x80) == 128;
        let ty = four[0] & 0x7F;
        let size = u32::from_be_bytes(const_array!(@start(0), four => 1, 2, 3));
        let block_buf = stream.take(size as usize)?;
        if block_buf.len() != size as usize {
            return Err(InvalidFormat);
        }
        if B::BLOCK_TYPE == ty {
            println!("3333---");
            return Ok(Some(B::from_bytes(block_buf.to_owned())?));
        }
        if end {
            break Ok(None);
        }
    }
}
pub(crate) struct Stream<'a> {
    inner: &'a [u8],
    index: usize,
}

impl<'a> Stream<'a> {
    pub(crate) fn new(buf: &[u8]) -> Stream {
        Stream {
            inner: buf,
            index: 0,
        }
    }
    pub(crate) fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        let start = self.index;
        self.index += n;
        if self.index > self.inner.len() {
            return Err(InvalidFormat);
        }
        Ok(&self.inner[start..self.index])
    }
}
