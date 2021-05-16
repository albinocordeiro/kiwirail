use clap::Clap;
use color_eyre::eyre::Result;
use petgraph::graphmap::DiGraphMap;
use kiwirail::input_parsing::*;
use kiwirail::shortest_route::*;


fn main() -> Result<()> {
    color_eyre::install()?;
    println!(r#"
                    _          
    |/  o     o    |_) _  o  | 
    |\  | \^/ |    | \(_| |  |                                              
"#);

    let options = Options::parse();
    let graph_map: DiGraphMap<char, i32> = from_kiwi_file(&options.edges_file)?;

    match options.api {
        Api::RouteCount(query) => {
            let (origin, destination) = parse_node_pair(&query.town_pair)?; 
            match query.stop_condition {
                StopCondition::Upto(condition) => {
                    println!("Count routes from {} to {} with a maximum of {} stops", &origin, &destination, &condition.stops);
                },
                StopCondition::Exact(condition) => {
                    println!("Count routes from {} to {} with exactly {} stops", &origin, &destination, &condition.stops);
                }
            };
        },
        Api::RouteDistance(query) => {
            let route: Vec<char> = parse_route(&query.route)?;
            println!("Compute total route distance for route {:?}", &route);
        },
        Api::ShortestRoute(query) => {
            let (origin, destination) = parse_node_pair(&query.town_pair)?;
            println!("Compute shortest route between {} and {}", &origin, &destination);
            match dijkstra_shortest_route(&graph, origin, destination) {
                Ok(route_option) => {
                    match route_option {
                        Some(route) => println!("{:?}", route),
                        None => println!("NO SUCH ROUTE")
                    } 
                },
                Err(r) => {
                    return Err(r);
                }
            };

        },                               
    };

    Ok(())
}