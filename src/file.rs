use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use clap::ArgMatches;

pub fn file_path_from_clap(sub_m: &ArgMatches, clap_id: &str) -> Result<PathBuf, Box<dyn Error>> {
    let file_str = sub_m
        .value_of(clap_id)
        .expect(&format!("can't find the flag: {}", clap_id));

    let file_path = Path::new(file_str)
        .canonicalize()
        .expect(&format!("Can't find absolute path of file tag {}", clap_id));

    info!("Found File w/ tag \"{}\" at {:?}", clap_id, file_path);
    Ok(file_path)
}

pub fn bufreader_from_filepath(path: PathBuf) -> Result<BufReader<File>, Box<dyn Error>> {
    Ok(BufReader::new(File::open(path)?))
}
