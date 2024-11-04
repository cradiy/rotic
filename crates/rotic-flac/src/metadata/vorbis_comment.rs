use std::collections::HashMap;

static GARBLED: char = '�';

fn remove_non_printing_char(pat: &str) -> String {
    pat.chars()
        .filter(|ch| !format!("{:?}", ch).starts_with("'\\u"))
        .collect()
}

use super::ConvertBytes;
#[derive(Clone)]
pub struct VorbisComment {
    refer: String,
    inner: HashMap<String, String>,
    raw: Vec<u8>,
}

impl std::fmt::Debug for VorbisComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VorbisComment")
            .field("refer", &self.refer)
            .field("inner", &self.inner)
            .finish()
    }
}

impl VorbisComment {
    
    pub fn refer(&self) -> &str {
        &self.refer
    }
    /// Unprocessed Vorbis Comment
    pub fn raw_vorbis_comment(&self) -> &[u8] {
        &self.raw
    }
    pub fn vorbis_comment(&self) -> &HashMap<String, String> {
        &self.inner
    }
    pub fn title(&self) -> Option<&str> {
        self.inner.get("TITLE").map(|s| s.as_str())
    }
    pub fn year(&self) -> Option<u32> {
        self.inner.get("DATE")?.parse().ok()
    }
    pub fn album(&self) -> Option<&str> {
        self.inner.get("ALBUM").map(|s| s.as_str())
    }
    pub fn artist(&self) -> Option<&str> {
        self.inner.get("ARTIST").map(|s| s.as_str())
    }
    pub fn album_artist(&self) -> Option<&str> {
        self.inner.get("ALBUMARTIST").map(|s| s.as_str())
    }
    pub fn lyrics(&self) -> Option<&str> {
        self.inner.get("LYRICS").map(|s| s.as_str())
    }
}

impl ConvertBytes for VorbisComment {
    fn from_bytes(buf: Vec<u8>) -> crate::Result<Self> {
        let vorbis = String::from_utf8_lossy(&buf).replace(GARBLED, " ");
        let mut refer = String::new();
        let mut map: HashMap<String, String> = HashMap::new();
        for comment in vorbis.split('\0') {
            let trim = remove_non_printing_char(comment.trim());
            if trim.is_empty() {
                continue;
            }
            if trim.starts_with("reference") {
                refer = trim
            } else {
                let key_value: Vec<_> = trim.splitn(2, '=').collect();
                if key_value.len() != 2 {
                    continue;
                }
                let value = match key_value[0] {
                    "DATE" => key_value[1]
                        .chars()
                        .filter(|ch| ch.is_ascii_digit())
                        .collect(),
                    _ => key_value[1].trim().replace('"', ""),
                };
                map.insert(key_value[0].to_owned(), value);
            }
        }
        Ok(Self {
            refer,
            inner: map,
            raw: buf,
        })
    }

    fn into_bytes(self) -> Vec<u8> {
        let mut buf = Vec::new();
        if !self.refer.is_empty() {
            append(&mut buf, self.refer.as_bytes());
        }
        self.inner.iter().for_each(|(k, v)| {
            append(&mut buf, format!("{k}={v}").as_bytes())
        });
        buf
    }
}

fn append(buf: &mut Vec<u8>, content: &[u8]) {
    buf.extend_from_slice(content);
    buf.push(0);
    buf.push(0);
}