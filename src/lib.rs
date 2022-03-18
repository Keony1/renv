use std::fs;
use std::io::prelude::*;
use std::{env, error::Error};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.sourcefile).expect("Arquivo não encontrado");

    let json_pattern = json_patten(&contents);
    let sls_pattern = sls_pattern(&contents);

    println!("{json_pattern}\n");
    println!("{sls_pattern}\n");

    if config.export == "true" {
        let mut file = fs::File::create("variables.txt").unwrap();
        file.write(sls_pattern.as_bytes());
        file.write("\n\n".as_bytes());
        file.write(json_pattern.as_bytes());
    }

    Ok(())
}

#[derive(Debug)]
pub struct Config {
    sourcefile: String,
    export: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let sourcefile = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a sourcefile"),
        };

        Ok(Config {
            sourcefile,
            export: args.next().unwrap_or_default(),
        })
    }
}

fn sls_pattern(contents: &String) -> String {
    let lines = contents.lines();
    let mut sls_format = String::from("environment:\n");

    for line in lines {
        if line == "" {
            continue;
        }

        let mut key_value = line.split("=");
        let key = key_value.nth(0).unwrap();

        let pattern = format!("   {key}: ${{env:{key}, self:custom.secrets.{key}}}\n").to_owned();
        sls_format.push_str(pattern.as_str());
    }

    sls_format
}

fn json_patten(contents: &String) -> String {
    let lines = contents.lines();

    let mut json_format = String::from("{\n");

    for line in lines {
        if line == "" {
            continue;
        }

        let key_value: Vec<&str> = line.split("=").collect();
        let key = key_value[0];
        let value = key_value[1];

        json_format.push_str(format!("   \"{key}\": \"{value}\"\n").as_str());
    }

    json_format.push_str("}");
    json_format
}
