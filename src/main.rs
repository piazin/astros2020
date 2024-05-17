extern crate serde;
extern crate serde_yaml;

use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
struct Config {
    config: Rule,
}
#[derive(Debug, Deserialize)]
struct Rule {
    rule: String,
    rule2: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("./src/astros.yml")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Erro ao ler o arquivo");

    let config: Config = serde_yaml::from_str(&contents).expect("Erro ao fazer parsing do YAML");

    println!("{:?}", config);

    Ok(())
}
