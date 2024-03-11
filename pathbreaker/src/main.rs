mod utils;

use utils::read_blinded_path_from_file;

fn main() {
    // Path to the JSON file
    let file_path = "./data/blinded_path.json";

    match read_blinded_path_from_file(file_path) {
        Ok(blinded_path) => {
            println!("Introduction Node: {}", blinded_path.introduction_node);
            println!("Blinded Nodes: {:?}", blinded_path.blinded_nodes);
            println!("Fee Base (msat): {}", blinded_path.fee_base_msat);
            println!("Fee Proportional Millionths: {}", blinded_path.fee_proportional_millionths);
            println!("HTLC Minimum (msat): {}", blinded_path.htlc_minimum_msat);
            println!("CLTV Expiry Delta: {}", blinded_path.cltv_expiry_delta);
            println!("Max CLTV Expiry: {}", blinded_path.max_cltv_expiry);
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}