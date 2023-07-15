use petgraph::graphmap::UnGraphMap;
use std::hash::Hash;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct City {
    population: u32,
    cars: u32,
}

fn main() {
    let mut graph = UnGraphMap::<_, ()>::new();
    let bedford_falls = City {
        population: 1023,
        cars: 24,
    };
    let tinsel_town = City {
        population: 102479,
        cars: 1231441,
    };

    graph.add_node(&bedford_falls);
    graph.add_node(&tinsel_town);
    graph.add_edge(&bedford_falls, &tinsel_town, ());

    assert!(graph.contains_node(&bedford_falls));
    assert!(graph.contains_node(&tinsel_town));
    assert!(graph.contains_edge(&bedford_falls, &tinsel_town));
    assert!(graph.contains_edge(&tinsel_town, &bedford_falls));
}
