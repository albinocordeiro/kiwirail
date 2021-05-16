use color_eyre::eyre::Result;
use petgraph::Graph;
use petgraph::graphmap::DiGraphMap;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use std::collections::HashMap;

pub fn dijkstra_shortest_route(graph: &DiGraphMap<char, i32>, origin: char, destination: char) -> Result<Option<Vec<char>>> {
    let route: Vec<char> = Vec::new();
    let res = dijkstra(graph, origin , Some(destination), |e| *e.weight());
    Ok(Some(route))
}