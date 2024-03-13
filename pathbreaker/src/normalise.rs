extern crate statrs;
extern crate nalgebra;

use std::collections::HashMap;

use crate::{channel_route::{Constraints, Route}, utils::BlindedPath};
use nalgebra::DVector;
use statrs::distribution::{MultivariateNormal, Continuous};

/// Approach
/// Create sets of functions are struct that takes input a vector of route, and real constraints as input
/// And find the most likely candidate for the end point.

/// Functions
/// 1. A function that takes input a set of constraints and standardize them, so that they are of comparable range
/// 2. A function that takes the standardize parameters as input, and create a well-tuned multivariate normal distribution for it.
/// 3. A function that takes input a route, create standardized parameter from it's constraints, use it to create multivariate normal distribution
///    and calculate PD (Probability Density) for a provided set of standardized constraints.
/// 4. A function that takes a route, and the real constraints and find the most likely candidate for the final being the final node.

impl Constraints {
    pub fn standardize(&self) -> Vec<f64> {
        // Define the mean and standard deviation for each field
        const PATH_LENGTH_MEAN: f64 = 10.0;
        const PATH_LENGTH_STDDEV: f64 = 10.0;
        const FEE_BASE_MSAT_MEAN: f64 = 2000.0;
        const FEE_BASE_MSAT_STDDEV: f64 = 1000.0;
        const HTLC_MINIMUM_MSAT_MEAN: f64 = 1500.0;
        const HTLC_MINIMUM_MSAT_STDDEV: f64 = 500.0;
        const CLTV_EXPIRY_DELTA_MEAN: f64 = 80.0;
        const CLTV_EXPIRY_DELTA_STDDEV: f64 = 20.0;

        // Apply standardization formula to each field
        let standardized_path_length = (self.path_length as f64 - PATH_LENGTH_MEAN) / PATH_LENGTH_STDDEV;
        let standardized_fee_base_msat = (self.fee_base_msat as f64 - FEE_BASE_MSAT_MEAN) / FEE_BASE_MSAT_STDDEV;
        let standardized_htlc_minimum_msat = (self.htlc_minimum_msat as f64 - HTLC_MINIMUM_MSAT_MEAN) / HTLC_MINIMUM_MSAT_STDDEV;
        let standardized_cltv_expiry_delta = (self.cltv_expiry_delta as f64 - CLTV_EXPIRY_DELTA_MEAN) / CLTV_EXPIRY_DELTA_STDDEV;

        // Return standardized constraints
        vec![
            standardized_path_length,
            standardized_fee_base_msat,
            standardized_htlc_minimum_msat,
            standardized_cltv_expiry_delta
        ]
    }
}

fn create_distribution(mean_vector: Vec<f64>) -> Result<MultivariateNormal, statrs::StatsError> {
    // Create a covariance matrix
    let covariance_matrix = vec![
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ]; // Assuming ideal scenario of no codependence for now.

    // Create the multivariate normal distribution
    MultivariateNormal::new(mean_vector, covariance_matrix)
}

pub fn get_route_probability(route: &Route, real_constraints: &Constraints) -> f64 {
    let expected_constraints = route.constraints.standardize();
    let real_constraints = real_constraints.standardize();
    let d_vec = DVector::from(real_constraints);

    if let Ok(mv_normal) = create_distribution(expected_constraints) {
        mv_normal.pdf(&d_vec)
    } else {
        // Handle error when creating distribution fails
        0.0
    }
}

pub fn get_most_probable_receiver(routes: Vec<Route>, blinded_path: BlindedPath) -> Vec<(String, f64)> {
    let real_constraints = blinded_path.constraints();
    let mut node_probabilities: HashMap<String, f64> = HashMap::new();

    for route in routes {
        let receiver = route.nodes.last().unwrap().clone();
        let route_probability = get_route_probability(&route, &real_constraints);
        let total_prob = node_probabilities.entry(receiver).or_insert(0.0);
        * total_prob += route_probability;
    }

    let mut sorted_receiver:Vec<(String, f64)> = node_probabilities.into_iter().collect();
    sorted_receiver.sort_by(|(_, probs1), (_, probs2)| probs2.partial_cmp(probs1).unwrap_or(std::cmp::Ordering::Equal));

    sorted_receiver
}