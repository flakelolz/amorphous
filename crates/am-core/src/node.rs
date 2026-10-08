use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Blueprint, BlueprintId, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(Uuid);

impl NodeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    /// Node's unique identifier.
    id: NodeId,
    /// Node's name. Initially inherited from the blueprint.
    name: String,
    /// Blueprint ID used to create the node.
    blueprint: BlueprintId,
    /// Node's properties. Initially inherited from the blueprint.
    properties: IndexMap<String, Value>,
    /// Node's children.
    children: Vec<Node>,
}

impl Node {
    /// Create a new node from a blueprint, inheriting the blueprint's name and properties.
    pub fn new(blueprint: &Blueprint) -> Self {
        Self {
            id: NodeId::new(),
            name: blueprint.name().into(),
            blueprint: blueprint.id(),
            properties: blueprint.properties().clone(),
            children: Vec::new(),
        }
    }

    /// Create a new node with the given name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Create a new node with a given property.
    pub fn with_property(mut self, name: impl Into<String>, value: Value) -> Self {
        self.set_property(name, value);
        self
    }

    /// Create a new node with a given child.
    pub fn with_child(mut self, child: Node) -> Self {
        self.add_child(child);
        self
    }

    /// Set the name of the node.
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// Insert a property into the node.
    pub fn set_property(&mut self, name: impl Into<String>, value: Value) -> Option<Value> {
        self.properties.insert(name.into(), value)
    }

    /// Get a property from the node.
    pub fn get_property(&self, name: &str) -> Option<&Value> {
        self.properties.get(name)
    }

    /// Get a mutable property from the node.
    pub fn get_property_mut(&mut self, name: &str) -> Option<&mut Value> {
        self.properties.get_mut(name)
    }

    /// Remove a property from the node.
    pub fn remove_property(&mut self, name: &str) -> Option<Value> {
        self.properties.shift_remove(name)
    }

    /// Add a child to the node.
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node() {
        let bp = Blueprint::new("Hitbox");
        let mut node = Node::new(&bp);
        assert_eq!(node.name, "Hitbox");

        node.set_property("Damage", Value::Int(10));
        assert_eq!(node.get_property("Damage"), Some(&Value::Int(10)));

        node.add_child(Node::new(&bp).with_name("Child"));
        assert_eq!(node.children.len(), 1);
    }

    #[test]
    fn with_name() {
        let bp = Blueprint::new("");
        let node = Node::new(&bp).with_name("Hitbox");
        assert_eq!(node.name, "Hitbox");
    }

    #[test]
    fn with_property() {
        let bp = Blueprint::new("Animation").with_property("Length", Value::Int(21));
        let node = Node::new(&bp).with_property("Repeat", Value::Bool(true));
        assert_eq!(node.name, "Animation");
        assert_eq!(node.get_property("Length"), Some(&Value::Int(21)));
        assert_eq!(node.get_property("Repeat"), Some(&Value::Bool(true)));
    }
}
