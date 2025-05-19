use petgraph::algo::ford_fulkerson;
use petgraph::prelude::*;

fn main() {
    ford_fulkerson_gaps_in_edge_index_test();
}

fn ford_fulkerson_gaps_in_edge_index_test() {
    let mut g: StableGraph<(), u32, Directed> = StableDiGraph::new();

    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    let d = g.add_node(());

    let ac = g.add_edge(a, c, 1);
    let ab = g.add_edge(a, b, 1);
    let bc = g.add_edge(b, c, 1);
    let cd = g.add_edge(c, d, 1);

    // Current state of graph:
    // a --1-- b --1-- c --1-- d
    // |               |
    // - -- -- 1 -- -- -

    println!("Edges before removing bc: {:?}", g.edge_indices().collect::<Vec<_>>());
    // Output: Edges before removing bc: [EdgeIndex(0), EdgeIndex(1), EdgeIndex(2), EdgeIndex(3)]
    g.remove_edge(bc);
    println!("Edges after removing bc: {:?}", g.edge_indices().collect::<Vec<_>>());
    // Output: Edges after removing bc: [EdgeIndex(0), EdgeIndex(1), EdgeIndex(3)]

    // Current state of graph:
    // a --1-- b       c --1-- d
    // |               |       
    // - -- -- 1 -- -- -

    println!("Graph: {:?}", g);
    // Output: Graph: StableGraph { Ty: "Directed", node_count: 4, edge_count: 3, edges: (0, 2), (0, 1), (1, 3), edge weights: {0: 1, 1: 1, 3: 1}, free_node: NodeIndex(4294967295), free_edge: EdgeIndex(2) 

    let flow = ford_fulkerson(&g, a, d);
    // thread 'main' panicked at src/algo/ford_fulkerson.rs:75:72:
    // index out of bounds: the len is 3 but the index is 3
    println!("Flow: {:?}", flow);
}