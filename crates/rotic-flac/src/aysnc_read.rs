use crate::error::Error::*;
use crate::metadata::{Block, BlockBytes};
use crate::{const_array, Result};
use tokio::io::{AsyncRead, AsyncReadExt};

pub async fn read_from_async_stream<R: AsyncRead + Unpin>(
    stream: &mut R,
) -> Result<Vec<BlockBytes>> {
    let mut blocks = Vec::new();
    let mut four = [0; 4];
    stream.read_exact(&mut four).await?;
    if &four != b"fLaC" {
        return Err(InvalidFormat);
    }

    loop {
        stream.read_exact(&mut four).await?;
        let end = (four[0] & 0x80) == 128;
        let ty = four[0] & 0x7F;
        let size = u32::from_be_bytes(const_array!(@start(0), four => 1, 2, 3));
        let mut block_buf = Vec::new();
        stream.take(size as u64).read_to_end(&mut block_buf).await?;
        if block_buf.len() != size as usize {
            return Err(InvalidFormat);
        }
        blocks.push(Block::new(end, ty, size, block_buf));
        if end {
            break Ok(blocks);
        }
    }
}
