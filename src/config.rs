use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

#[cfg(not(any(feature = "config_toml")))]
const _: () = panic!("At least one config format needs to be enabled (recommended: config_toml)");

pub trait Format {
    fn serialize<T: serde::Serialize>(value: &T) -> Vec<u8>;
    fn deserialize<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> T;
}

#[cfg(feature = "config_toml")]
pub struct Toml;

#[cfg(feature = "config_toml")]
impl Format for Toml {
    fn serialize<T: serde::Serialize>(value: &T) -> Vec<u8> {
        toml::to_string(value).unwrap().into_bytes()
    }

    fn deserialize<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> T {
        toml::from_slice(bytes).unwrap()
    }
}

pub trait Config: Default + Serialize + for<'a> Deserialize<'a> {
    type Format: Format;

    fn save<P: AsRef<Path>>(&self, path: &P) {
        let contents = Self::Format::serialize(&self);
        fs::write(path, contents).expect("failed to write");
    }

    fn load<P: AsRef<Path>>(path: &P) -> Self {
        if fs::exists(path).expect("failed to check exists") {
            let contents = fs::read_to_string(path).expect("failed to write");
            Self::Format::deserialize(contents.as_bytes())
        } else {
            let value = Self::default();
            value.save(path);
            value
        }
    }
}
