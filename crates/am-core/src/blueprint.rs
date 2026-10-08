use crate::Value;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlueprintId(Uuid);

impl BlueprintId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Blueprint {
    id: BlueprintId,
    name: String,
    properties: IndexMap<String, Value>,
}

impl Blueprint {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: BlueprintId::new(),
            name: name.into(),
            properties: IndexMap::new(),
        }
    }

    pub fn id(&self) -> BlueprintId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn properties(&self) -> &IndexMap<String, Value> {
        &self.properties
    }

    pub fn with_property(mut self, name: impl Into<String>, value: Value) -> Self {
        self.properties.insert(name.into(), value);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Blueprints(IndexMap<BlueprintId, Blueprint>);
