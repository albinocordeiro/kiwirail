//! Count total number of distinct routes with three flavours of queries supported:
//! exact number of stops, maximum number of stops and max distance.
use petgraph::graphmap::DiGraphMap;
use petgraph::algo::all_simple_paths;
use crate::route_distance::*;

/// Variation of the standard Depth First Search traversal where 
pub fn count_routes_with_exact_num_stops(graph: &DiGraphMap<char, i32>, origin: char, 
                                            destination: char, stops: i32) -> i32 {
    let intermediate_stops = (stops - 1) as usize;
    let paths_collection: Vec<Vec<char>> = all_simple_paths(graph, origin, destination, intermediate_stops, Some(intermediate_stops)).collect();
    
    paths_collection.len() as i32
}

pub fn count_routes_with_max_stops (graph: &DiGraphMap<char, i32>, origin: char, destination: char, stops: i32) -> i32 {
    let paths_collection: Vec<Vec<char>> = all_simple_paths(graph, origin, destination, 0, Some(stops as usize)).collect();

    paths_collection.len() as i32
}

pub fn count_routes_with_max_total_distance(graph: &DiGraphMap<char, i32>, origin: char, destination: char, distance_limit: i32) -> i32 {
    let all_paths: Vec<Vec<char>> = all_simple_paths(graph, origin, destination, 0, None).collect();
    let mut filtered_count: i32 = 0;

    for path in all_paths {
        if let Some(path_distance) = distance_along_route(graph, &path) {
            if path_distance < distance_limit {
                filtered_count += 1;
            }
        }
    }

    filtered_count
}