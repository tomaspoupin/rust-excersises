use std::collections::HashMap;
use graphs::types;

fn main() 
{
    let mut graph_list = HashMap::new();

    graph_list.insert('A', vec!['B', 'D']);
    graph_list.insert('B', vec!['C']);
    graph_list.insert('C', vec!['A', 'D', 'E']);
    graph_list.insert('D', vec!['E', 'F']);
    graph_list.insert('E', vec!['B', 'G']);
    graph_list.insert('F', vec!['G']);
    graph_list.insert('G', vec![]);
    
    let graph = types::Graph::new(graph_list);

    for node in graph.depth_iter('B')
    {
        println!("Node: {}", node);
    }
}
