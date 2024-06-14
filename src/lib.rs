use clap::Parser;

use crate::cli_build::Cli;

mod cli_build;

pub fn init_0(){
    let cli = Cli::parse();
    dbg!(cli);
}

