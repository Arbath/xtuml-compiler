use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::{Result, Context};

#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    #[serde(default)]
    pub model_name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub classes: Vec<ClassDef>,
    #[serde(default)]
    pub events: Vec<EventDef>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClassDef {
    pub name: String,
    #[serde(default)]
    pub domain_ref: String,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
    #[serde(default)]
    pub states: Vec<StateDef>,
    #[serde(default)]
    pub methods: Vec<MethodDef>,
    #[serde(default)]
    pub state_model: Option<Vec<StateModel>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attribute {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StateDef {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MethodDef {
    pub name: String,
    #[serde(default)]
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventDef {
    pub name: String,
    pub trigger: Option<String>,
    #[serde(default)]
    pub action: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StateModel {
    pub initial_state: String,
    pub states: Vec<State>,
    pub transitions: Vec<Transition>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transition {
    pub from: String,
    pub event: String,
    pub to: String,
    pub action: Option<String>,
}


pub fn load_model<P: AsRef<Path>>(path: P) -> Result<Model> {
    let s = fs::read_to_string(&path)
        .with_context(|| format!("failed to read model file {}", path.as_ref().display()))?;
    let model: Model = serde_json::from_str(&s).context("failed to parse JSON model")?;
    Ok(model)
}
