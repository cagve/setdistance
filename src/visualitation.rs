use petgraph::{prelude::Graph, dot::Dot};

pub fn view(metric_space:Vec<Vec<String>>) {
    let mut g : Graph<&str, &str> = Graph::new();
    metric_space.iter()
        .for_each(|set|{
            set.iter().for_each(|val|{
                g.add_node(val);
            })
        });
    println!("{}", Dot::new(&g));
}


