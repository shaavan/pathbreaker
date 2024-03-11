use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct BlindedPath {
    pub introduction_node: String,
    pub blinded_nodes: Vec<String>,
    pub fee_base_msat: u64,
    pub fee_proportional_millionths: u64,
    pub htlc_minimum_msat: u64,
    pub cltv_expiry_delta: u32,
    pub max_cltv_expiry: u32,
}

pub fn read_blinded_path_from_file(file_path: &str) -> Result<BlindedPath, Box<dyn Error>> {
    // Open the file in read-only mode
    let path = Path::new(file_path);
    let mut file = File::open(&path)?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON string into a BlindedPath object
    let blinded_path: BlindedPath = serde_json::from_str(&contents)?;

    Ok(blinded_path)
}