//! Count total number of distinct routes with three flavours of queries supported:
//! exact number of stops, maximum number of stops and max distance.
use petgraph::graphmap::DiGraphMap;


pub fn count_routes_with_exact_num_stops(graph: &DiGraphMap<char, i32>, origin: char, 
                                            destination: char, stops: i32) -> i32 {
    let filtered_count: i32 = 0;
                       

    filtered_count
}