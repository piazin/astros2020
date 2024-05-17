use serde_yaml;

use crate::FileConfig;
use anyhow::{Context, Ok, Result};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_config_file<P: AsRef<Path> + Copy>(path: P, contents: &mut String) -> Result<()> {
    let mut file =
        File::open(path).with_context(|| format!("Failed to open file: {:?}", path.as_ref()))?;

    file.read_to_string(contents)
        .with_context(|| format!("Failed to read file: {:?}", path.as_ref()))?;

    Ok(())
}

pub fn parse_config_file(config_content: &mut String) -> Result<FileConfig> {
    let config: FileConfig =
        serde_yaml::from_str(&config_content).with_context(|| format!("Failed to parse file"))?;

    Ok(config)
}

#[test]
pub fn test_read_config_file() -> Result<()> {
    let path: &str = "./src/astros.yml";
    let mut contents = String::new();
    read_config_file(path, &mut contents)?;

    assert_eq!(contents.contains("strategie"), true);
    assert_eq!(contents.contains("duration"), true);
    assert_eq!(contents.contains("attack_per_second"), true);
    assert_eq!(contents.contains("objective"), true);

    Ok(())
}

#[test]
fn test_parse_config_file() -> Result<()> {
    let path: &str = "./src/astros.yml";
    let mut contents = String::new();
    read_config_file(&path, &mut contents)?;
    let FileConfig { config, scenarios } = parse_config_file(&mut contents)?;

    assert_eq!(config.strategie.duration, 10);
    assert_eq!(config.strategie.attack_per_second, 20);
    assert_eq!(config.objective, "http://localhost:3000/api");

    if let Some(get) = scenarios[0].flow[0].get.as_ref() {
        assert_eq!(get.url, "/users");
    }

    Ok(())
}
