use crate::Value;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlueprintId(Uuid);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Blueprint {
    id: BlueprintId,
    name: String,
    properties: IndexMap<String, Value>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Blueprints(IndexMap<BlueprintId, Blueprint>);
