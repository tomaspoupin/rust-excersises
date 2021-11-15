pub mod types
{
    use std::collections::HashMap;
    use std::hash::Hash;

    pub struct Graph<N: Eq + Hash>  
    {
        data: HashMap<N, Vec<N> >,
    }

    pub struct GraphIter<'a, N: Eq + Hash>
    {
        graph: &'a HashMap<N, Vec<N> >,
        visited: HashMap<&'a N, bool>,
        stack: Vec<&'a N>,
    }

    impl<N: Eq + Hash> Graph<N>
    {
        pub fn new(data: HashMap<N, Vec<N> >) -> Graph<N>
        {
            Graph 
            {
                data,
            }
        }

        pub fn depth_iter(&self, idx: N) -> GraphIter<N>
        {
            GraphIter::new(&self.data, idx)
        }
    }

    impl<'a, N: Eq + Hash> GraphIter<'a, N>
    {
        fn new(graph: &'a HashMap<N, Vec<N> >, idx: N) -> GraphIter<'a, N> 
        {
            let mut new_graph_iter = GraphIter {
                graph,
                visited: HashMap::new(),
                stack: Vec::new(),
            };

            for (idx, _) in new_graph_iter.graph
            {
                new_graph_iter.visited.insert(idx, false);
            }

            let (key, _) = new_graph_iter.graph.get_key_value(&idx).unwrap();

            new_graph_iter.stack.push(key);
            new_graph_iter
        }
    }
    
    impl<'a, N: Eq + Hash> Iterator for GraphIter<'a, N>
    {
       type Item = &'a N; 

       fn next(&mut self) -> Option<Self::Item>
       {
            let entry = self.stack.pop();
            match entry
            {
                Some(idx) => {
                    self.visited.insert(idx, true);
                    for node in self.graph.get(idx).unwrap()
                    {
                        if *self.visited.get(node).unwrap() == false
                        {
                            self.visited.insert(node, true);
                            self.stack.push(node);
                        }
                    }
                    Some(idx)
                },

                None => None,
            }
       }
    }
}
