mod utils;
mod channel_route;

use utils::read_channel_graph_from_file;

use crate::{channel_route::{find_end_points, get_final_routes}, utils::read_blinded_path_from_file};

fn main() {
    // Assuming channel_graph and other variables are defined
    let channel_graph = read_channel_graph_from_file("data/test_channel_graph.json").unwrap();
    let blinded_path = read_blinded_path_from_file("data/test_blinded_path.json").unwrap();
    let final_routes = get_final_routes(&channel_graph, &blinded_path);
    println!("{:?}", find_end_points(&final_routes))
}
