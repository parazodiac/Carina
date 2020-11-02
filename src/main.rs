extern crate clap;
extern crate pretty_env_logger;
extern crate rust_htslib;

#[macro_use]
extern crate log;

use clap::{App, Arg, SubCommand};
use std::error::Error;

mod utils;
mod cbcount;
mod samplebam;

pub const CB_LENGTH: usize = 16;
pub const MIL: usize = 1_000_000;
pub const TMIL: usize = 10_000_000;
pub const HMIL: usize = 100_000_000;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("carina")
        .version("0.1.0")
        .author("Avi Srivastava")
        .about("A set of fast helper functions for everyday use.")
        .subcommand(
            SubCommand::with_name("cbcount")
                .about("A subcommand to generate the frequency distribution of CB.")
                .arg(
                    Arg::with_name("ibam")
                        .long("ibam")
                        .short("i")
                        .takes_value(true)
                        .required(true)
                        .help("path to the BAM file"),
                )
                .arg(
                    Arg::with_name("otsv")
                        .long("otsv")
                        .short("o")
                        .takes_value(true)
                        .required(true)
                        .help("path to the output tsv file"),
                )
                .arg(
                    Arg::with_name("tenx")
                        .long("tenx")
                        .help("use tag CB from 10x generated BAM."),
                )
        )
        .subcommand(
            SubCommand::with_name("samplebam")
                .about("A subcommand to subsample bam based on CB.")
                .arg(
                    Arg::with_name("ibam")
                        .long("ibam")
                        .short("i")
                        .takes_value(true)
                        .required(true)
                        .help("path to the BAM file"),
                )
                .arg(
                    Arg::with_name("icb")
                        .long("icb")
                        .short("c")
                        .takes_value(true)
                        .required(true)
                        .help("path to the input list of CB"),
                )
        )
        .get_matches();
    pretty_env_logger::init_timed();

    if let Some(sub_m) = matches.subcommand_matches("cbcount") {
        cbcount::callback(&sub_m)?
    }

    if let Some(sub_m) = matches.subcommand_matches("samplebam") {
        samplebam::callback(&sub_m)?
    }

    Ok(())
}