use serde::{Serialize};

#[derive(Serialize)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Serialize)]
pub struct Node {
    pub id: String,
    pub r#type: String,
    pub label: String,
}

#[derive(Serialize)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub r#type: String,
}