use std::{cmp::Ordering, collections::HashMap, ops::Add};

use crate::utils::{BlindedPath, ChannelGraph, NodePolicy};

/// A route is a list of channels with the latest node, and channel_id
#[derive(Debug, Default, Clone)]
pub struct Route {
    nodes: Vec<String>, // A, B, C
    channel_ids: Vec<String>, // Channel A-B, Channel B-C
    constraints: Constraints,
}

impl Route {
    fn create_new_route(&self, next_node: &String, next_channel_id: &String, next_constraints: Constraints) -> Route {
        let mut new_nodes = self.nodes.clone();
        new_nodes.push(next_node.clone());
        let mut new_channel_ids = self.channel_ids.clone();
        new_channel_ids.push(next_channel_id.clone());
        let new_route_constraints = self.constraints.clone() + next_constraints;

        Route {
            nodes: new_nodes,
            channel_ids: new_channel_ids,
            constraints: new_route_constraints
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Constraints {
    path_length: u32,
    fee_base_msat: u64,
    htlc_minimum_msat: u64,
    cltv_expiry_delta: u32,
}

impl Default for Constraints {
    fn default() -> Self {
        Constraints {
            path_length: 1,
            fee_base_msat: 0,
            htlc_minimum_msat: 0,
            cltv_expiry_delta: 0,
        }
    }
}

impl PartialOrd for Constraints {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Constraints {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.path_length > other.path_length ||
            self.fee_base_msat > other.fee_base_msat ||
            self.htlc_minimum_msat > other.htlc_minimum_msat ||
            self.cltv_expiry_delta > other.cltv_expiry_delta {
            Ordering::Greater
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

impl Add for Constraints {
    type Output = Constraints;

    fn add(self, other: Constraints) -> Constraints {
        Constraints {
            path_length: self.path_length + other.path_length,
            fee_base_msat: self.fee_base_msat + other.fee_base_msat,
            htlc_minimum_msat: self.htlc_minimum_msat.max(other.htlc_minimum_msat),
            cltv_expiry_delta: self.cltv_expiry_delta + other.cltv_expiry_delta,
            // TODO: Update fee_base_msat, and add fee_proportional_millionths if total amount forward can be predicted somehow
        }
    }
}

fn get_constraints_from_blinded_path(blinded_path: &BlindedPath) -> Constraints {
    Constraints {
        path_length: blinded_path.blinded_nodes.len() as u32 + 1, // Including the introduction node
        fee_base_msat: blinded_path.fee_base_msat,
        htlc_minimum_msat: blinded_path.htlc_minimum_msat,
        cltv_expiry_delta: blinded_path.cltv_expiry_delta,
    }
}

fn get_constraints_from_node_policy(node_policy: &NodePolicy) -> Constraints {
    Constraints {
        path_length: 1,
        fee_base_msat: node_policy.fee_base_msat.parse::<u64>().unwrap(),
        htlc_minimum_msat: node_policy.min_htlc.parse::<u64>().unwrap(),
        cltv_expiry_delta: node_policy.time_lock_delta,
    }
}

pub fn shared_edge_node_policies(channel_graph: &ChannelGraph, node_pub: &str) -> Vec<(String, String, NodePolicy)> {
    let mut shared_edge_node_n_policy = Vec::new();

    for edge in &channel_graph.edges {
        if node_pub == &edge.node1_pub {
            shared_edge_node_n_policy.push((edge.node2_pub.clone(), edge.channel_id.clone(), edge.node2_policy.clone()));
        } else if node_pub == &edge.node2_pub {
            shared_edge_node_n_policy.push((edge.node1_pub.clone(), edge.channel_id.clone(), edge.node2_policy.clone()));
        }
    }
    shared_edge_node_n_policy
}

fn next_good_routes(current_route: &Route, channel_graph: &ChannelGraph, real_constraints: &Constraints) -> Option<Vec<Route>> {
    let last_node_pub = current_route.nodes.last().unwrap();
    let next_node_policies = shared_edge_node_policies(channel_graph, last_node_pub);

    let mut next_routes = Vec::new();

    for (next_node, next_channel_id, next_channel_policy) in next_node_policies.iter() {
        // Condition 1 => The next_node is not already present in the route (avoiding circular loops)
        if current_route.nodes.contains(next_node) {
            continue;
        }

        // Condition 2 => Constraints for the added path are satisfied
        let next_channel_constraints = get_constraints_from_node_policy(next_channel_policy);
        let sum_constraints = current_route.constraints + next_channel_constraints;
        if sum_constraints < real_constraints.clone() {
            let new_route = current_route.create_new_route(next_node, next_channel_id, next_channel_constraints);
            next_routes.push(new_route);
        }
    }

    // That means the route is complete
    if next_routes.is_empty() {
        return None
    } else {
        return Some(next_routes)
    }
}

fn get_good_routes(final_routes: &mut Vec<Route>, cur_route: &Route, channel_graph: &ChannelGraph, real_constraints: &Constraints) {
    // For the current route find the next good routes. That is route that satisfy the parent, and value constraints
    if let Some(next_routes) = next_good_routes(cur_route, channel_graph, real_constraints) {
        for next_route in next_routes {
            get_good_routes(final_routes, &next_route, channel_graph, real_constraints)
        }
    }
    // If no such routes are found for a cur_route, that means the route is completed and can be added to final_routes
    else {
        final_routes.push(cur_route.clone())
    }
}

pub fn get_final_routes(channel_graph: &ChannelGraph, blinded_path: &BlindedPath) -> Vec<Route> {
    let intro_node = blinded_path.introduction_node.clone();
    let real_constraints = get_constraints_from_blinded_path(blinded_path);
    let mut final_routes = Vec::new();

    let first_route = Route {
        nodes: vec![intro_node],
        channel_ids: Vec::new(),
        constraints: Constraints::default(),
    };

    get_good_routes(&mut final_routes, &first_route, channel_graph, &real_constraints);

    final_routes
}

pub fn find_end_points(routes: &Vec<Route>) -> Vec<(String, usize)> {
    let mut node_counts: HashMap<String, usize> = HashMap::new();

    for route in routes {
        if let Some(last_node) = route.nodes.last() {
            let count = node_counts.entry(last_node.clone()).or_insert(0);
            *count += 1;
        }
    }

    let mut sorted_nodes: Vec<(String, usize)> = node_counts.into_iter().collect();
    sorted_nodes.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

    sorted_nodes
}


#[test]
fn test_constraint_ordering() {
    let constraint1 = Constraints { path_length: 3, fee_base_msat: 2000, htlc_minimum_msat: 1000, cltv_expiry_delta: 160 };
    let constraint2 = Constraints { path_length: 4, fee_base_msat: 2500, htlc_minimum_msat: 1000, cltv_expiry_delta: 150 };

    assert!(constraint1 > constraint2);
}