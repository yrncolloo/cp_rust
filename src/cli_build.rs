use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version = "0.0.1", name="cpR")]
#[command(about = "rust implementation of cp command", long_about = None)]
pub struct Cli{
    /// Same as -dR --preserve=all
    #[arg(short='a', long)]
    archive: bool,

    /// Don't copy the file data, just the attributes
    #[arg(long)]
    attributes_only: bool,

    /// Make a backup of each destination file
    #[arg(long, name="CONTROL")]
    backup: Option<PathBuf>,

    /// Like --backup but does not accept an argument
    #[arg(short='b')]
    backup_no_args: bool,

    /// Copy contents of special files when recusive
    #[arg(long)]
    copy_contents: bool,

    ///Same as --no-dereference --preserve-links
    #[arg(short='d')]
    deref: bool,

    /// Explains how file is copied. Implies -v
    #[arg(long)]
    debug: bool,




}
