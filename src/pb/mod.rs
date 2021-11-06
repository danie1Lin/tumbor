mod abi;
pub use abi::*;
use base64::{URL_SAFE_NO_PAD, decode_config, encode_config};
use prost::Message;


impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data= decode_config(value, URL_SAFE_NO_PAD)?;
        Ok(ImageSpec::decode(&data[..])?)
    }
}

impl From<ImageSpec> for String {
    fn from(value: ImageSpec) -> Self {
        
        let data = value.encode_to_vec();
        encode_config(data, URL_SAFE_NO_PAD)
    }
}