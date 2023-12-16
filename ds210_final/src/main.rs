use petgraph::graph::NodeIndex;
use std::collections::HashMap;
mod graph;

fn main() {
    let undirected_graph = graph::read_graph_from_file("roadNet-CA.txt");

    //calculate the top 10 intersections/endpoints with the most road connections
    let degrees = graph::calculate_degree(&undirected_graph);

    //the degree centrality for each of the top 10 intersections/endpoints (degrees/n-1)
    let degree_centrality = graph::calculate_degree_centrality(&undirected_graph);
    
    //prints the top 10 nodes by degrees and degree centrality
    top_nodes_centrality("Degree Centrality", &degree_centrality, 10);
    top_nodes_degrees("Degree Centrality", &degrees, 10);

    //average distance of a subset of 1000 intersections/endpoints
    let avg_distance = graph::calculate_average_distance(&undirected_graph, 1000);
    println!("Average Distance: {:.2}", avg_distance);

}

//function to print the degree centralities of the top 10 nodes
fn top_nodes_centrality(centrality_type: &str, centrality: &HashMap<NodeIndex, f64>, top_count: usize) {
    println!("Top 10 Nodes by {}: ", centrality_type);

    let mut sorted_nodes: Vec<_> = centrality.iter().collect();
    sorted_nodes.sort_by(|(_, &a), (_, &b)| b.partial_cmp(&a).unwrap());

    for (i, (&node, &value)) in sorted_nodes.iter().take(top_count).enumerate() {
        println!("{}. Node {}: {:.12}", i + 1, node.index(), value);
    }

    println!();
}

//function to print the degree for the top 10 nodes
fn top_nodes_degrees(centrality_type: &str, centrality: &HashMap<NodeIndex, f64>, top_count: usize) {
    println!("Top 10 Nodes by {}: ", centrality_type);

    let mut sorted_nodes: Vec<_> = centrality.iter().collect();
    sorted_nodes.sort_by(|(_, &a), (_, &b)| b.partial_cmp(&a).unwrap());

    for (i, (&node, &value)) in sorted_nodes.iter().take(top_count).enumerate() {
        println!("{}. Node {}: {:.2}", i + 1, node.index(), value);
    }

    println!();
}

