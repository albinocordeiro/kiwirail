use clap::Clap;
use color_eyre::eyre::Result;
use petgraph::graphmap::DiGraphMap;
use kiwirail::input_parsing::*;
use kiwirail::shortest_route::*;
use kiwirail::route_distance::*;
use kiwirail::route_count::*;


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
            let count: i32 = match query.stop_condition {
                StopCondition::Upto(condition) => {
                    println!("Count routes from {} to {} with a maximum of {} stops", &origin, &destination, &condition.stops);
                    count_routes_with_max_stops(&graph_map, origin, destination, condition.stops)
                },
                StopCondition::Exact(condition) => {
                    println!("Count routes from {} to {} with exactly {} stops", &origin, &destination, &condition.stops);
                    count_routes_with_exact_num_stops(&graph_map, origin, destination, condition.stops)
                },
                StopCondition::MaxDistance(condition) => {
                    println!("Count routes from {} to {} with a distance less than {}", &origin, &destination, &condition.distance_limit);
                    count_routes_with_max_total_distance(&graph_map, origin, destination, condition.distance_limit)
                    
                }
            };
            println!("Number of routes matching criteria: {}", count);
        },
        Api::RouteDistance(query) => {
            let route: Vec<char> = parse_route(&query.route)?;
            println!("Compute total route distance for route {:?}", &route);
            match distance_along_route(&graph_map, &route) {
                Some(total_distance) => println!("Total distance: {}", total_distance),
                None => println!("NO SUCH ROUTE")
            };
        },
        Api::ShortestRoute(query) => {
            let (origin, destination) = parse_node_pair(&query.town_pair)?;
            println!("Compute the length of the shortest route between {} and {}", &origin, &destination);
            match dijkstra_shortest_route_length(&graph_map, origin, destination) {
                Some(distance) => println!("Found min distance: {}", distance),
                None => println!("NO SUCH ROUTE")
            };
        },                               
    };

    Ok(())
}