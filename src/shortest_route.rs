//! Use Dijkstra's algorithm to compute the length of the shortest path between two towns
//! It treats the special case when the given origin and destination are the same in the following way:
//! We test all the possibilities of the traveler leaving the origin town and what would be the minimum
//! round trip back as opposed to just returning 0 (that is the traveler don't leave town). 
use petgraph::graphmap::DiGraphMap;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;

/// Compute the length of the shortest route between two towns
/// It treats the special case when the given origin and destination are the same in the following way:
/// We test all the possibilities of the traveler leaving the origin town and what would be the minimum
/// travel distance to come back as opposed of just returning 0, that is the traveler don't leave town.
pub fn dijkstra_shortest_route_length(graph: &DiGraphMap<char, i32>, origin: char, destination: char) -> Option<i32> {
    if origin == destination {
        let mut round_trip: i32 = i32::MAX;
        
        for neighbour in graph.neighbors_directed(origin, Direction::Outgoing) {
            if let Some(dist_to_neighbour) = graph.edge_weight(origin, neighbour) {
                // Recursive call!!
                if let Some(dist_to_return) = dijkstra_shortest_route_length(graph, neighbour, origin) {
                    round_trip = std::cmp::min(round_trip, dist_to_neighbour + dist_to_return);
                } 
            } else {
                panic!("Something's really really wrong. Fix the code!");
            }
        }
        if round_trip == i32::MAX {
            return None;
        }
        return Some(round_trip);
    }

    let res = dijkstra(graph, origin , Some(destination), |e| *e.weight());
    match res.get(&destination) {
        Some(distance_ref) => Some(*distance_ref),
        None => None
    }
}