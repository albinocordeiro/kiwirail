use clap::Clap;
use color_eyre::eyre::Result;
use color_eyre::eyre::eyre;
use regex::Regex;
use petgraph::graphmap::DiGraphMap;
use kiwirail::input_parsing::*;


#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Albino Cordeiro <albino@intuitionlogic.com>")]
struct Options {
    #[clap(subcommand)]
    api: Api,
    #[clap(short, long, required = true, 
        about = "One edge per line, using the format [origin][destination][distance]. Example: AC5")]
    edges_file: String,
    #[clap(short, long, required = false, parse(from_occurrences))]
    verbose: i32,
}

#[derive(Clap, Debug)]
enum Api {
    #[clap(about = "Will compute the cost (distance) of a given route.")]
    RouteDistance(DistanceQuery),
    #[clap(about = "Count the number of routes between two given towns. Accepts a bound on number of stops.")]
    RouteCount(CountQuery),
    #[clap(about = "Get the shortest route between two given towns.")]
    ShortestRoute(DijkstraQuery),
}

#[derive(Clap, Debug)]
struct DistanceQuery {
    #[clap(short, long, required = true)]
    route: String,
}

#[derive(Clap, Debug)]
struct CountQuery {
    #[clap(short, long, required = true, 
        about = "Pair of town in the format [Origin][Destination]. Example: AB")]
    town_pair: String,
    #[clap(subcommand)]
    stop_condition: StopCondition,
}

#[derive(Clap, Debug)]
enum StopCondition {
    Upto(UptoCondition),
    Exact(ExactCondition),
}

#[derive(Clap, Debug)]
struct UptoCondition {
    #[clap(about = "Maximum number of stops", required = true)]
    k: i32,
}

#[derive(Clap, Debug)]
struct ExactCondition {
    #[clap(about = "Exact number of stops", required = true)]
    k: i32,
}

#[derive(Clap, Debug)]
struct DijkstraQuery {
    #[clap(short, long, required = true,
        about = "Pair of town in the format [Origin][Destination]. Example: AB")]
    town_pair: String,
}

#[derive(Clap, Debug)]
struct GenerateParams {
    #[clap(short, long, required = true, about = "Number of towns (less than 26)")]
    number_of_towns: i32,
}

fn parse_node_pair(pair: &str) -> Result<(char, char)> {
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

fn parse_route(rstring: &str) -> Result<Vec<char>> {
    let re = Regex::new(r"^\b[A-Z]{2,}\b$")?;
    if !re.is_match(rstring) {
        return Err(eyre!("The input {} is not a valid route", rstring));
    }
    let res: Vec<char> = rstring.chars().collect();

    Ok(res)
}
fn main() -> Result<()> {
    color_eyre::install()?;
    println!(r#"
__________________________
                          
    /       ,            ,
---/-__-------------------
  /(      /   | /| /   /  
_/___\___/____|/_|/___/___
                          
                          
___________________________
    ____                   
    /    )           ,    /
---/___ /-----__---------/-
  /    |    /   )  /    /  
_/_____|___(___(__/____/___
                                                    
"#);

    let options = Options::parse();
    let graph_map: DiGraphMap<char, i32> = from_kiwi_file(&options.edges_file)?;


    match options.api {
        Api::RouteCount(query) => {
            let (origin, destination): (char, char) = parse_node_pair(&query.town_pair)?; 
            match query.stop_condition {
                StopCondition::Upto(condition) => {
                    println!("Count routes from {} to {} with a maximum of {} stops", origin, destination, condition.k)
                },
                StopCondition::Exact(condition) => {
                    println!("Count routes from {} to {} with a maximum of {} stops", origin, destination, condition.k)
                }
            }
        },
        Api::RouteDistance(query) => {
            let route: Vec<char> = parse_route(&query.route)?;
            println!("Compute total route distance for route {:?}", route);
        },
        Api::ShortestRoute(query) => {
            let (origin, destination) = parse_node_pair(&query.town_pair)?;
            println!("Compute shortest route between {} and {}", origin, destination);
        },                               
    };

    Ok(())
}