mod utils;
mod channel_route;
mod normalise;

use normalise::get_most_probable_receiver;
use utils::read_channel_graph_from_file;
use crate::{channel_route::get_final_routes, utils::read_blinded_path_from_file};

fn main() {
    // Assuming channel_graph and other variables are defined
    let channel_graph = read_channel_graph_from_file("test_vectors/test_channel_graph.json").unwrap();
    let blinded_path = read_blinded_path_from_file("test_vectors/test_blinded_path.json").unwrap();
    let final_routes = get_final_routes(&channel_graph, &blinded_path);

    let final_probs = get_most_probable_receiver(final_routes, blinded_path);

    println!("{:?}", final_probs)
}
