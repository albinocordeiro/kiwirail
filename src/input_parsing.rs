//let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
use color_eyre::eyre::{Result, eyre};
use petgraph::graphmap::DiGraphMap;
use regex::Regex;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{prelude::*, BufReader};


pub fn from_kiwi_file(file_name: &str) -> Result<DiGraphMap<char, i32>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    println!("Reading graph from file");
    let mut graph: DiGraphMap<char, i32> =  DiGraphMap::new();
    for line in reader.lines() {
        let estring = &(line?);
        let (n1, n2, d): (char, char, i32) = parse_edge(estring)?; 
        graph.add_edge(n1, n2, d);
    }
    println!("Done reading graph");

    Ok(graph)
}


fn parse_edge(edge_string: &str) -> Result<(char, char, i32)> {
    lazy_static! {
        static ref EDGE_RE: Regex = Regex::new(r"\b([A-Z])([A-Z])(\d{1,})\b").unwrap();
    }

    let captures = EDGE_RE.captures_iter(edge_string);
    for cap in captures {
        let node1 = *match &cap[1].chars().next() {
            Some(n) => n,
            None => return Err(eyre!("Couldn't extract node from {}", edge_string))
        };
        let node2 = *match &cap[2].chars().next() {
            Some(n) => n,
            None => return Err(eyre!("Couldn't extract node from {}", edge_string))
        };
        let distance = *&cap[3].parse::<i32>()?; 

        return Ok((node1, node2, distance));
    }

    Err(eyre!("Something really wrong happened while parsing an edge"))
}