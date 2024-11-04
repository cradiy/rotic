use std::path::Path;

use crate::{error::Error, Stream};

use super::ConvertBytes;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PictureType {
    Other,
    FileIcon,
    OtherFileIcon,
    CoverFont,
    CoverBack,
    LeafletPage,
    Media,
    Lead,
    Artist,
    Conductor,
    Band,
    Composer,
    Lyricist,
    RecordingLocation,
    DuringRecording,
    DuringPerformance,
    VideoScreenCapture,
    BrightColouredFish,
    Illustration,
    BandLogoType,
    PublisherLogoType,
}
impl PictureType {
    pub fn from_u8(value: u8) -> Option<PictureType> {
        let value = match value {
            0 => Self::Other,
            1 => Self::FileIcon,
            2 => Self::OtherFileIcon,
            3 => Self::CoverFont,
            4 => Self::CoverBack,
            5 => Self::LeafletPage,
            6 => Self::Media,
            7 => Self::Lead,
            8 => Self::Artist,
            9 => Self::Conductor,
            10 => Self::Band,
            11 => Self::Composer,
            12 => Self::Lyricist,
            13 => Self::RecordingLocation,
            14 => Self::DuringRecording,
            15 => Self::DuringPerformance,
            16 => Self::VideoScreenCapture,
            17 => Self::BrightColouredFish,
            18 => Self::Illustration,
            19 => Self::BandLogoType,
            20 => Self::PublisherLogoType,
            _ => return None,
        };
        Some(value)
    }
}
#[derive(Clone)]
pub struct Picture {
    ty: PictureType,
    mime: String,
    description: String,
    width: u32,
    height: u32,
    color_depth: u32,
    indexed_color_pictures: u32,
    picture: Vec<u8>,
}

impl std::fmt::Debug for Picture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Picture")
            .field("ty", &self.ty)
            .field("mime", &self.mime)
            .field("description", &self.description)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("color_depth", &self.color_depth)
            .field("indexed_color_pictures", &self.indexed_color_pictures)
            .field("picture", &"[..]")
            .finish()
    }
}

impl Picture {
    pub fn save_to_path<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        std::fs::write(path, &self.picture)
    }
    pub fn picture_type(&self) -> PictureType {
        self.ty
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn mime_type(&self) -> &str {
        &self.mime
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn color_depth(&self) -> u32 {
        self.color_depth
    }
    pub fn indexed_color_pictures(&self) -> u32 {
        self.indexed_color_pictures
    }
}
fn be_u32(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}
impl ConvertBytes for Picture {
    fn from_bytes(buf: Vec<u8>) -> crate::Result<Self> {
        let mut stream = Stream::new(&buf);
        let buf = stream.take(4)?;
        let Some(ty) = PictureType::from_u8(buf[3]) else {
            return Err(Error::InvalidFormat);
        };
        let mime_len = be_u32(stream.take(4)?) as usize;
        let mime = String::from_utf8_lossy(stream.take(mime_len)?).to_string();
        let description_len = be_u32(stream.take(4)?) as usize;
        let description = String::from_utf8_lossy(stream.take(description_len)?).to_string();

        let width = be_u32(stream.take(4)?);
        let height = be_u32(stream.take(4)?);
        let color_depth = be_u32(stream.take(4)?);
        let indexed_color_pictures = be_u32(stream.take(4)?);
        let picture_len = be_u32(stream.take(4)?) as usize;
        let picture = stream.take(picture_len)?.to_vec();
        Ok(Self {
            ty,
            mime,
            description,
            width,
            height,
            color_depth,
            indexed_color_pictures,
            picture,
        })
    }

    fn into_bytes(mut self) -> Vec<u8> {
        let mut bytes = vec![0, 0, 0, self.ty as _];
        bytes.extend_from_slice(&self.mime.len().to_be_bytes());
        bytes.append(&mut self.mime.into_bytes());
        bytes.extend_from_slice(&self.description.len().to_be_bytes());
        bytes.append(&mut self.description.into_bytes());
        bytes.extend_from_slice(&self.width.to_be_bytes());
        bytes.extend_from_slice(&self.height.to_be_bytes());
        bytes.extend_from_slice(&self.color_depth.to_be_bytes());
        bytes.extend_from_slice(&self.indexed_color_pictures.to_be_bytes());
        bytes.extend_from_slice(&self.picture.len().to_be_bytes());
        bytes.append(&mut self.picture);
        bytes
    }
}
