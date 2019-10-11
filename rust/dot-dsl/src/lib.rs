pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self
                }

                pub fn get_attr(&self, attr_name: &str) -> Option<&str> {
                    self.attrs.get(attr_name).map(|s| &**s)
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq)]
            pub struct Edge {
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    unimplemented!()
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self
                }
            }
        }
    }

    use graph_items::edge::Edge;
    use graph_items::node::Node;

    #[derive(Debug, PartialEq)]
    pub struct Graph<'a> {
        pub nodes: Vec<&'a Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            for node in nodes {
                self.nodes.push(node);
            }
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            unimplemented!()
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for attr in attrs {
                self.attrs.insert(attr.0.to_string(), attr.1.to_string());
            }
            self
        }

        pub fn get_node(&self, label: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == label)
        }

        pub fn get_attr(&self, attr_name: &str) -> Option<&String> {
            self.attrs.get(attr_name)
        }
    }
}
