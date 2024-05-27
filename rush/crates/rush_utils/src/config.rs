use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::Read,
    path::PathBuf
};
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RushConfigMemoryText {
    pub start: u32,
    pub end: u32
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RushConfigMemoryGlobal {
    pub bot: u32,
    pub ptr: u32
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RushConfigMemoryStack {
    pub top: u32,
    pub bot: u32
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RushConfigMemory {
    pub text: RushConfigMemoryText,
    pub global: RushConfigMemoryGlobal,
    pub data_bot: u32,
    pub heap_bot: u32,
    pub stack: RushConfigMemoryStack
}

/// # The user's Rush configuration.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RushConfig {
    pub memory: RushConfigMemory,
    pub executable: String,
    pub start_addr: u32
}

impl RushConfig {
    pub fn new(config_path_str: &String) -> Self {
        let config_path = PathBuf::from(config_path_str);

        let mut file = File::open(&config_path).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let config = serde_yaml::from_str(&contents).unwrap();

        return config
    }
}




