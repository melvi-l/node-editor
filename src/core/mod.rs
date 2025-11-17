use serde::{Deserialize, Serialize};

type PortId = String;
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Port {
    id: PortId,
    name: String,
}
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
struct PortRef {
    node_id: NodeId,
    port_id: PortId,
}
type Output = Port;
type Input = Port;

type NodeId = String;
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Node {
    id: NodeId,
    kind: String,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
}

type EdgeId = String;
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Edge {
    id: EdgeId,
    from: PortRef,
    to: PortRef,
}

#[derive(thiserror::Error, Debug)]
pub enum GraphError {
    #[error("Unknown node")]
    UnknownNode,
    #[error("Unknown edge")]
    UnknownEdge,
    #[error("Unknown port")]
    UnknownPort,
    #[error("Wrong direction")]
    WrongDirection,
    #[error("Type mismatch")]
    TypeMismatch,
    #[error("Input already connected")]
    InputOccupied,
    #[error("Cycle detected")]
    Cycle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn create_node(&mut self, node: Node) -> Result<(), GraphError> {
        self.nodes.push(node);
        Ok(())
    }

    pub fn delete_node(&mut self, node_id: NodeId) -> Result<Node, GraphError> {
        if let Some(index) = self.nodes.iter().position(|node| node.id == node_id) {
            Ok(self.nodes.swap_remove(index))
        } else {
            Err(GraphError::UnknownNode)
        }
    }

    pub fn connect(
        &mut self,
        from: PortRef,
        to: PortRef,
        edge_id: EdgeId,
    ) -> Result<(), GraphError> {
        self.nodes
            .iter()
            .find(|node| node.id == from.node_id)
            .ok_or(GraphError::UnknownNode)?
            .outputs
            .iter()
            .find(|output| output.id == from.port_id)
            .ok_or(GraphError::UnknownPort)?;

        self.nodes
            .iter()
            .find(|node| node.id == to.node_id)
            .ok_or(GraphError::UnknownNode)?
            .inputs
            .iter()
            .find(|input| input.id == to.port_id)
            .ok_or(GraphError::UnknownPort)?;

        if self.edges.iter().any(|edge| edge.to == to) {
            return Err(GraphError::InputOccupied);
        }

        self.edges.push(Edge {
            id: edge_id,
            from,
            to,
        });
        Ok(())
    }

    pub fn disconnect(&mut self, edge_id: EdgeId) -> Result<Edge, GraphError> {
        if let Some(index) = self.edges.iter().position(|edge| edge.id == edge_id) {
            Ok(self.edges.swap_remove(index))
        } else {
            Err(GraphError::UnknownEdge)
        }
    }
}

pub fn test_core() -> Result<(), GraphError> {
    let mut graph = Graph {
        nodes: vec![Node {
            id: "node_1".to_string(),
            kind: "idk".to_string(),
            inputs: vec![Port {
                id: "input_1".to_string(),
                name: "idk".to_string(),
            }],
            outputs: vec![Port {
                id: "output_1".to_string(),
                name: "idk".to_string(),
            }],
        }],
        edges: Vec::new(),
    };
    graph.create_node(Node {
        id: "node_2".to_string(),
        kind: "idk".to_string(),
        inputs: vec![Port {
            id: "input_2".to_string(),
            name: "idk".to_string(),
        }],
        outputs: vec![Port {
            id: "output_2".to_string(),
            name: "idk".to_string(),
        }],
    })?;
    graph.connect(
        PortRef {
            node_id: "node_1".to_string(),
            port_id: "output_1".to_string(),
        },
        PortRef {
            node_id: "node_2".to_string(),
            port_id: "input_2".to_string(),
        },
        "edge_1".to_string(),
    )?;
    graph.connect(
        PortRef {
            node_id: "node_2".to_string(),
            port_id: "output_2".to_string(),
        },
        PortRef {
            node_id: "node_1".to_string(),
            port_id: "input_1".to_string(),
        },
        "edge_2".to_string(),
    )?;
    graph.disconnect("edge_1".to_string());
    Ok(())
}
