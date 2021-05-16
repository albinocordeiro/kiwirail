//! Everything input parsing  
use color_eyre::eyre::{Result, eyre};
use petgraph::graphmap::DiGraphMap;
use regex::Regex;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use clap::Clap;

/// Main command line argument options and help system
#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Albino Cordeiro <albino@intuitionlogic.com>")]
pub struct Options {
    #[clap(subcommand)]
    pub api: Api,
    #[clap(short, long, required = true, 
        about = "One edge per line, using the format [origin][destination][distance]. Example: AC5")]
    pub edges_file: String,
    #[clap(short, long, required = false, parse(from_occurrences))]
    pub verbose: i32,
}

/// The kiwicli functionality is provided by three sub-commands
#[derive(Clap, Debug)]
pub enum Api {
    #[clap(about = "Will compute the cost (distance) of a given route.")]
    RouteDistance(DistanceQuery),
    #[clap(about = "Count the number of routes between two given towns. Accepts a bound on number of stops.")]
    RouteCount(CountQuery),
    #[clap(about = "Get the shortest route between two given towns.")]
    ShortestRoute(DijkstraQuery),
}

#[derive(Clap, Debug)]
pub struct DistanceQuery {
    #[clap(short, long, required = true)]
    pub route: String,
}

#[derive(Clap, Debug)]
pub struct CountQuery {
    #[clap(short, long, required = true, 
        about = "Pair of town in the format [Origin][Destination]. Example: AB")]
    pub town_pair: String,
    #[clap(subcommand)]
    pub stop_condition: StopCondition,
}

/// Two flavours of route count queries
#[derive(Clap, Debug)]
pub enum StopCondition {
    #[clap(about = "Return routes with a maximum number of stops")]
    Upto(StopParam),
    #[clap(about = "Return routes with an exact number of stops")]
    Exact(StopParam),
}

#[derive(Clap, Debug)]
pub struct StopParam {
    #[clap(required = true)]
    pub stops: i32,
}

#[derive(Clap, Debug)]
pub struct DijkstraQuery {
    #[clap(short, long, required = true,
        about = "Pair of town in the format [Origin][Destination]. Example: AB")]
    pub town_pair: String,
}

pub fn parse_node_pair(pair: &str) -> Result<(char, char)> {
    let re = Regex::new(r"\b[A-Z]{2}\b")?;
    if !re.is_match(pair) {
        return Err(eyre!("The input {} is not a valid origin/destination pair. Usage example: --town-pair AB ."));
    }
    let mut chars = pair.chars();
    if let (Some(origin), Some(destination)) = (chars.next(), chars.next()) {
        return Ok((origin, destination));
    }

    Err(eyre!("Could not extract node pair from {}", pair))
}

pub fn parse_route(rstring: &str) -> Result<Vec<char>> {
    let re = Regex::new(r"^\b[A-Z]{2,}\b$")?;
    if !re.is_match(rstring) {
        return Err(eyre!("The input {} is not a valid route", rstring));
    }
    let res: Vec<char> = rstring.chars().collect();

    Ok(res)
}

/// Expects a file in the kiwi format, that is, a weighted edge per line that look like the following
/// AB10
/// AC80
/// BH40
/// ...
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