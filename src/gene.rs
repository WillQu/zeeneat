use crate::sigmoid::sigmoid;
use rand::seq::SliceRandom;
use rand::random;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct NodeId(u32);
#[derive(Clone, Copy, Debug)]
enum NodeType {
    Input,
    Output,
    Hidden,
}
#[derive(Clone, Copy, Debug)]
struct NodeGene {
    id: NodeId,
    node_type: NodeType,
}

#[derive(Clone, Copy, Debug)]
struct ConnectionGene {
    in_node: NodeId,
    out_node: NodeId,
    weight: f64,
    enabled: bool,
}
#[derive(Clone, Debug)]
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
        println!("Initial conections: {:?}", connections);
        Genome {
            nodes,
            connections,
        }
    }

    pub(crate) fn calculate_fitness(&self) -> f64 {
        struct IO {
            input: (f64, f64),
            expected_output: f64,
        }
        let inputs = vec![
            IO { input: (0.0, 0.0), expected_output: 0.0 },
            IO { input: (0.0, 1.0), expected_output: 1.0 },
            IO { input: (1.0, 0.0), expected_output: 1.0 },
            IO { input: (1.0, 1.0), expected_output: 0.0 },
        ];
        let total_error = inputs.iter().fold(0.0, |acc, elem| {
            let output = self.calculate_output(elem.input.0, elem.input.1);
            let error = (elem.expected_output - output).abs();
            acc + error
        });
        1.0 / (total_error + 1e-18)
    }

    fn calculate_node(&self, node_id: NodeId, calculated_nodes: &mut HashMap<NodeId, f64>) -> f64 {
        calculated_nodes.get(&node_id).map(|v| *v).unwrap_or_else(|| {
            let connections = self.connections.iter().filter(|c| c.out_node == node_id && c.enabled);
            calculated_nodes.insert(node_id, 0.0);
            let sum = connections.fold(0.0, |acc, c| {
                let in_value = self.calculate_node(c.in_node, calculated_nodes);
                acc + in_value * c.weight
            });
            calculated_nodes.insert(node_id, sum);
            sigmoid(sum)
        })
    }

    pub(crate) fn calculate_output(&self, a: f64, b: f64) -> f64 {
        let mut calculated_nodes = HashMap::new();
        calculated_nodes.insert(NodeId(0), a);
        calculated_nodes.insert(NodeId(1), b);
        self.calculate_node(NodeId(2), &mut calculated_nodes)
    }

    fn mutate_connection_status(&self) -> Genome {
        let mut new_genome = self.clone();
        let connection = new_genome.connections.choose_mut(&mut rand::thread_rng()).unwrap();
        connection.enabled = !connection.enabled;
        new_genome
    }

    fn mutate_perturbate_connection_weight(&self) -> Genome {
        let mut new_genome = self.clone();
        let connection = new_genome.connections.choose_mut(&mut rand::thread_rng()).unwrap();
        connection.weight += random::<f64>() * 0.2 - 0.1;
        new_genome
    }

    fn mutate_replace_connection_weight(&self) -> Genome {
        let mut new_genome = self.clone();
        let connection = new_genome.connections.choose_mut(&mut rand::thread_rng()).unwrap();
        connection.weight = random::<f64>() * 2.0 - 1.0;
        new_genome
    }

    fn mutate_add_connection(&self) -> Genome {
        let mut new_genome = self.clone();
        let mut rng = rand::thread_rng();
        let in_node = *new_genome.nodes.choose(&mut rng).unwrap();
        let out_node = *new_genome.nodes.choose(&mut rng).unwrap();
        let new_connection = ConnectionGene {
            in_node: in_node.id,
            out_node: out_node.id,
            weight: random::<f64>() * 2.0 - 1.0,
            enabled: true,
        };
        new_genome.connections.push(new_connection);
        new_genome
    }

    fn mutate_add_node(&self) -> Genome {
        let mut new_genome = self.clone();
        let connection = new_genome.connections.choose_mut(&mut rand::thread_rng()).unwrap();
        connection.enabled = false;
        let new_node = NodeGene {
            id: NodeId(new_genome.nodes.len() as u32),
            node_type: NodeType::Hidden,
        };
        let new_connection1 = ConnectionGene {
            in_node: connection.in_node,
            out_node: new_node.id,
            weight: 1.0,
            enabled: true,
        };
        let new_connection2 = ConnectionGene {
            in_node: new_node.id,
            out_node: connection.out_node,
            weight: connection.weight,
            enabled: true,
        };
        new_genome.nodes.push(new_node);
        new_genome.connections.push(new_connection1);
        new_genome.connections.push(new_connection2);
        new_genome
    }

    pub(crate) fn mutate(&self) -> Genome {
        match random::<u32>() % 5 {
            0 => self.mutate_connection_status(),
            1 => self.mutate_perturbate_connection_weight(),
            2 => self.mutate_replace_connection_weight(),
            3 => self.mutate_add_connection(),
            4 => self.mutate_add_node(),
            _ => unreachable!(),
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
}