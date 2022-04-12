use clap::Parser;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(parse(from_os_str), short)]
    filesource: std::path::PathBuf,

    #[clap(short)]
    export: Option<String>,
}

pub fn run(cli: Cli) {
    let mut reader_json = get_file_reader(&cli.filesource);
    let mut reader_sls = get_file_reader(&cli.filesource);

    let json_pattern = json_pattern(&mut reader_json);
    let sls_pattern = sls_pattern(&mut reader_sls);

    if cli.export.is_some() {
        let mut file = File::create("variables.txt").unwrap();
        file.write(sls_pattern.as_bytes());
        file.write("\n\n".as_bytes());
        file.write(json_pattern.as_bytes());
    }

    println!("{json_pattern}\n");
    println!("{sls_pattern}\n");
}

fn get_file_reader(path: &std::path::PathBuf) -> impl BufRead {
    let result = File::open(path);

    let file = match result {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Problem parsing arguments: {}", error);
            process::exit(1);
        }
    };

    BufReader::new(file)
}

fn sls_pattern<R: BufRead>(reader: &mut R) -> String {
    let lines = reader.lines();

    let mut sls_format = String::from("environment:\n");
    for line in lines {
        let line = line.unwrap();
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

fn json_pattern<R: BufRead>(reader: &mut R) -> String {
    let lines = reader.lines();

    let mut json_format = String::from("{\n");

    let mut iterator = lines.peekable();

    while let Some(line) = iterator.next() {
        let line = line.unwrap();
        if line == "" {
            continue;
        }

        let key_value: Vec<&str> = line.split("=").collect();
        let key = key_value[0];
        let value = key_value[1];

        json_format.push_str(format!("   \"{key}\": \"{value}\"").as_str());

        if iterator.peek().is_some() {
            json_format.push_str(",\n");
        }

        if iterator.peek().is_none() {
            break;
        }
    }

    json_format.push_str("\n}");
    json_format
}
