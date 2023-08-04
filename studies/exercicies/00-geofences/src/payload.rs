use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Comand {
    pub command: String,
    pub group: String,
    pub detect: String,
    pub hook: String,
    pub key: String,
    pub time: String,
    pub id: String,
    pub object: Object,
}

#[derive(Debug, Deserialize)]
pub struct Object {
    pub type_field: String,
    pub coordinates: Vec<f64>,
}
