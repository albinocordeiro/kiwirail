//! Count total number of distinct routes with three flavours of queries supported:
//! exact number of stops, maximum number of stops and max distance.
use petgraph::graphmap::DiGraphMap;
use petgraph::prelude::*;


fn exact_stops_dfs(g: &DiGraphMap<char, i32>, curr_node: char, target_node: char, stops: i32, count: &mut i32) {
    if stops == 0 {
        if curr_node == target_node {
            *count += 1;
        } 
        return;
    }

    let neighbours = g.neighbors_directed(curr_node, Direction::Outgoing);
    for node in neighbours {
        exact_stops_dfs(g, node, target_node, stops - 1, count);
    }

}

/// Variation of the standard Depth First Search traversal where we tolerate revisiting nodes and the 
/// criteria for identifying the end of a path is the length of the path in terms of edges traversed
pub fn count_routes_with_exact_num_stops(graph: &DiGraphMap<char, i32>, origin: char, 
                                            destination: char, stops: i32) -> i32 {

    let mut path_count: i32 = 0;
    exact_stops_dfs(graph, origin, destination, stops, &mut path_count);
    
    path_count
}


fn max_stops_dfs(g: &DiGraphMap<char, i32>, curr_node: char, target_node: char, stops: i32, count: &mut i32) {
    if curr_node == target_node {
        *count += 1;
    }
    if stops == 0 {
        return;
    }

    let neighbours = g.neighbors_directed(curr_node, Direction::Outgoing);
    for node in neighbours {
        exact_stops_dfs(g, node, target_node, stops - 1, count);
    }
}

/// Variation of the standard Depth First Search traversal where we tolerate revisiting nodes and the 
/// criteria for incrementing the count is when we reach the target node, however, the path only ends when
/// maximum number of stops is reached
pub fn count_routes_with_max_stops (graph: &DiGraphMap<char, i32>, origin: char, destination: char, stops: i32) -> i32 {
    let mut path_count: i32 = 0;

    max_stops_dfs(graph, origin, destination, stops, &mut path_count);
    
    path_count
}

fn max_distance_dfs(g: &DiGraphMap<char, i32>, curr_node: char, target_node: char, distance: i32, count: &mut i32) {
    
    let neighbours = g.neighbors_directed(curr_node, Direction::Outgoing);
    for node in neighbours {
        
        match g.edge_weight(curr_node, node) {
            Some(edge_dist) => {
                if distance - *edge_dist > 0 {
                    if node == target_node {
                        *count += 1;
                    }
                    max_distance_dfs(g, node, target_node, distance - *edge_dist, count);
                }
            },
            None => panic!("This should never happen!"),
        };

    }
}

pub fn count_routes_with_max_total_distance(graph: &DiGraphMap<char, i32>, origin: char, destination: char, distance_limit: i32) -> i32 {
    let mut path_count: i32 = 0;

    max_distance_dfs(graph, origin, destination, distance_limit, &mut path_count);
    
    path_count
}