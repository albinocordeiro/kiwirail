//! Follow the route as given, don't do any extra stops. 
//! So if for the route A-B-C A,B or B,C are not neighbours, then return None

use petgraph::graphmap::DiGraphMap;

pub fn distance_along_route(graph: &DiGraphMap<char, i32>, route: &[char]) -> Option<i32> {
    if route.len() < 2 {
        return Some(0i32);
    }
    let mut res: i32 = 0;
    let mut start = route[0];
    for i in 1..route.len() {
        match graph.edge_weight(start, route[i]) {
            Some(valref) => {
                res += *valref;
                start = route[i];
            },
            None => return None
        };
    }

    Some(res)
}