use std::collections::HashMap;
use rand::random;
use crate::sigmoid::sigmoid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct NodeId(u32);
enum NodeType {
    Input,
    Output,
    Hidden,
}
struct NodeGene {
    id: NodeId,
    node_type: NodeType,
}

struct ConnectionGene {
    in_node: NodeId,
    out_node: NodeId,
    weight: f64,
    enabled: bool,
}
pub struct Genome {
    nodes: Vec<NodeGene>,
    connections: Vec<ConnectionGene>,
}

impl Genome {
    pub(crate) fn new(input_size: u32, output_size: u32) -> Genome {
        let mut nodes = Vec::new();
        for i in 0..input_size {
            nodes.push(NodeGene { id: NodeId(i), node_type: NodeType::Input });
        }
        for i in 0..output_size {
            nodes.push(NodeGene { id: NodeId(input_size + i), node_type: NodeType::Output });
        }
        let mut connections = Vec::new();
        for i in 0..input_size {
            for j in 0..output_size {
                connections.push(ConnectionGene {
                    in_node: NodeId(i),
                    out_node: NodeId(input_size + j),
                    weight: random::<f64>() * 2.0 - 1.0,
                    enabled: true,
                });
            }
        }
        Genome {
            nodes,
            connections: Vec::new(),
        }
    }

    pub(crate) fn calculate_fitness(&self) -> f64 {
        struct IO {
            input: (i32, i32),
            expected_output: i32,
        }
        let inputs = vec![
            IO { input: (0, 0), expected_output: 0 },
            IO { input: (0, 1), expected_output: 1 },
            IO { input: (1, 0), expected_output: 1 },
            IO { input: (1, 1), expected_output: 0 },
        ];
        let total_error = inputs.iter().fold(0i32, |acc, elem| {
            let output = self.calculate_output(elem.input.0, elem.input.1);
            let error = (elem.expected_output - output).abs();
            acc + error
        });
        1.0 / (total_error as f64 + 1e-6)
    }

    fn calculate_node(&self, node_id: NodeId, calculated_nodes: &mut HashMap<NodeId, f64>) -> f64 {
        calculated_nodes.get(&node_id).map(|v| *v).unwrap_or_else(|| {
            let connections = self.connections.iter().filter(|c| c.out_node == node_id && c.enabled);
            let sum = connections.fold(0.0, |acc, c| {
                let in_value = self.calculate_node(c.in_node, calculated_nodes);
                acc + in_value * c.weight
            });
            calculated_nodes.insert(node_id, sum);
            sigmoid(sum)
        })
    }

    fn calculate_output(&self, a: i32, b: i32) -> i32 {
        let mut calculated_nodes = HashMap::new();
        calculated_nodes.insert(NodeId(0), a as f64);
        calculated_nodes.insert(NodeId(1), b as f64);
        let output = self.calculate_node(NodeId(2), &mut calculated_nodes);
        if output > 0.5 {
            1
        } else {
            0
        }
    }
}