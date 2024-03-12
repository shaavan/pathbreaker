# Documentation for Process, Notes, and Progress

## Approach

For the given blinded path, and channel graph

1. Get the introduction node
2. `Find_channel_route_parameters` through the introduction node using the channel graph.
3. For each route, find cummulative `fee_base_msat`, `fee_propotional_millionth`, `htlc_minimum_msat`, `cltv_expiry_delta`
4. For each route, using the blinded path parameters, find the probability of the route using `Multiparameter Normal Distribution` (normalize the parameter first to have them in a similar range).
5. Divide all the probabilities with the sum of all probs, to make cumulative probs = 1.
6. Add the adjusted probability of routes that end on the same node. (Different routes are independent events)
7. Print the list of nodes with significant probability along with their adjusted probabilities.

## Functions to implement

- [x] Introduce functions to read the `blinded_path.json` and `channel_graph.json`
- [x] A function that takes complete `channel_graph`, `Intro node`, and `Max length of path (N)` as input, and outputs possible `channel route`s of N length.
    1. The `channel route` contains all the relevant information (1. fee_base_msat, 2. fee_proptional_millionth, 3. htlc_minimum_msat, 4. cltv_expiry_delta) in the direction payment forward.
- [x] Functions that for a given `channel route` find the cumulative:
    - [x] `fee_base_msat`
    - [x] `fee_propotional_millionth`
    - [x] `htlc_minimum_msat`
    - [x] `cltv_expiry_delta`
- [ ] A function that takes a list of properties as input and creates an adjusted multivariate normal distribution graph for them.
- [ ] A function that for a given `channel route` takes the expected cumulated properties as input, and calculated probability distribution factor for the value seen in reality (the original input values)
- [ ] // _Other functions as needed_