use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileConfig {
    pub config: Configs,
    pub scenarios: Vec<Scenarios>,
}

#[derive(Debug, Deserialize)]
pub struct Configs {
    pub objective: String,
    pub strategie: Strategie,
}

#[derive(Debug, Deserialize)]
pub struct Strategie {
    pub duration: u32,
    pub attack_per_second: u32,
}

#[derive(Debug, Deserialize)]
pub struct Scenarios {
    pub flow: Vec<Flow>,
}

#[derive(Debug, Deserialize)]
pub struct Flow {
    pub post: Option<Post>,
    pub get: Option<Get>,
}

#[derive(Debug, Deserialize)]
pub struct Post {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Get {
    pub url: String,
}
