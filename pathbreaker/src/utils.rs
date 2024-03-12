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

#[derive(Debug, Deserialize)]
pub struct ChannelGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Deserialize)]
pub struct Node {
    pub last_update: i64,
    pub pub_key: String,
    pub alias: String,
    pub addresses: Vec<Address>,
    pub color: String,
    pub features: serde_json::Value,
    pub custom_records: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct Address {
    pub network: String,
    pub addr: String,
}

#[derive(Debug, Deserialize)]
pub struct Edge {
    pub channel_id: String,
    pub chan_point: String,
    pub last_update: i64,
    pub node1_pub: String,
    pub node2_pub: String,
    pub capacity: String,
    pub node1_policy: NodePolicy,
    pub node2_policy: NodePolicy,
    pub custom_records: serde_json::Value,
}
#[derive(Debug, Deserialize, Clone)]
pub struct NodePolicy {
    pub time_lock_delta: u32,
    pub min_htlc: String,
    pub fee_base_msat: String,
    pub fee_rate_milli_msat: String,
    pub disabled: bool,
    pub max_htlc_msat: String,
    pub last_update: u64,
    pub custom_records: serde_json::Value,
}

pub fn read_channel_graph_from_file(file_path: &str) -> Result<ChannelGraph, Box<dyn Error>> {
    let path = Path::new(file_path);
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let channel_graph: ChannelGraph = serde_json::from_str(&contents)?;
    Ok(channel_graph)
}

#[allow(unused)]
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
