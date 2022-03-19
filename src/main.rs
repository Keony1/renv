use clap::StructOpt;
use renv::{ Cli };

fn main() {
    let args = Cli::parse();
    
    renv::run(args);
}
