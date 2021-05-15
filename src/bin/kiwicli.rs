use clap::Clap;
use color_eyre::eyre::Result;
use color_eyre::eyre::eyre;
use regex::Regex;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Albino Cordeiro <albino@intuitionlogic.com>")]
struct Options {
    #[clap(subcommand)]
    api: Api,
    #[clap(short, long, required = true, 
        about = "One edge per line, using the format [origin][destination][distance]. Example: AC5", 
        default_value = "test_data/graph.csv")]
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
    ShortestsRoute(DijkstraQuery),
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

fn parse_node_pair(pair: &str) -> Result<(char, char)> {
    if pair.len() != 2usize {
        return Err(eyre!("Provided town pair should be a string with the upper case characters"));
    }
    let chars = pair.chars();
    if let (Some(origin), Some(destination)) = (chars.next(), chars.next()) {
        return Ok((origin, destination));
    }

    Err(eyre!("Could not extract node pair from {}", pair))
}

fn parse_route(rstring: &str) -> Result<Vec<char>> {
    let re = Regex::new(r"^[A-Z]{2,}$")?;

    if re.find(rstring) {
        return Err(eyre!("Can't build route from {}", rstring));
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
        },
        Api::ShortestsRoute(query) => {

        },                               
    };

        Ok(())
}