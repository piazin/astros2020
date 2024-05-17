mod structs;
mod utils;
use anyhow::{Ok, Result};
use structs::*;
use utils::{parse_config_file, read_config_file};

fn main() -> Result<()> {
    let mut config_content = String::new();
    read_config_file("./src/astros.yml", &mut config_content)?;
    let FileConfig { config, scenarios } = parse_config_file(&mut config_content)?;

    println!("{}", config.objective);
    execute_scenario(&scenarios);

    Ok(())
}

fn execute_scenario(scenraios: &Vec<Scenarios>) {
    for scenario in scenraios {
        execute_flows(&scenario.flow);
    }
}

fn execute_flows(flow: &Vec<Flow>) {
    for f in flow {
        if let Some(post) = f.post.as_ref() {
            println!("Post: {:?}", post);
        }
        if let Some(get) = f.get.as_ref() {
            println!("Get: {:?}", get);
        }
    }
}
