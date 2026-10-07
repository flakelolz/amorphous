use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{BlueprintId, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(Uuid);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    id: NodeId,
    name: String,
    blueprint_id: BlueprintId,
    properties: IndexMap<String, Value>,
    children: Vec<Node>,
}
