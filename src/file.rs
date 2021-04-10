use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
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

pub fn try_file_path_from_clap(sub_m: &ArgMatches, clap_id: &str) -> Option<PathBuf> {
    let file_str = match sub_m.value_of(clap_id) {
        Some(val) => val,
        None => return None,
    };

    let file_path = Path::new(file_str)
        .canonicalize()
        .expect(&format!("Can't find absolute path of file tag {}", clap_id));

    info!("Found File w/ tag \"{}\" at {:?}", clap_id, file_path);
    Some(file_path)
}

pub fn files_path_from_clap(
    sub_m: &ArgMatches,
    clap_id: &str,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let files_str: Vec<&str> = sub_m
        .values_of(clap_id)
        .expect(&format!("can't find the flag: {}", clap_id))
        .collect();

    let mut files_path = Vec::new();
    for file_str in files_str {
        let file_path = Path::new(file_str)
            .canonicalize()
            .expect(&format!("Can't find absolute path of file tag {}", clap_id));
        files_path.push(file_path);
    }

    info!("Found File(s) w/ tag \"{}\" at {:?}", clap_id, files_path);
    Ok(files_path)
}

pub fn bufreader_from_filepath(path: PathBuf) -> Result<BufReader<File>, Box<dyn Error>> {
    Ok(BufReader::new(File::open(path)?))
}

pub fn bufwriter_from_clap(
    sub_m: &ArgMatches,
    clap_id: &str,
) -> Result<BufWriter<File>, Box<dyn Error>> {
    let file_str = sub_m
        .value_of(clap_id)
        .expect(&format!("can't find the flag: {}", clap_id));

    let file = BufWriter::new(File::create(file_str)?);
    let file_path = Path::new(file_str)
        .canonicalize()
        .expect(&format!("Can't find absolute path of file tag {}", clap_id));

    info!("Created File w/ tag \"{}\" at {:?}", clap_id, file_path);

    Ok(file)
}

pub fn bufwriter_from_clap_with_suffix(
    sub_m: &ArgMatches,
    clap_id: &str,
    suffix: &str,
) -> Result<BufWriter<File>, Box<dyn Error>> {
    let file_str = sub_m
        .value_of(clap_id)
        .expect(&format!("can't find the flag: {}", clap_id));

    let file_str = &format!("{}.{}.is", file_str, suffix);

    let file = BufWriter::new(File::create(file_str)?);
    let file_path = Path::new(file_str)
        .canonicalize()
        .expect(&format!("Can't find absolute path of file tag {}", clap_id));

    info!("Created File w/ tag \"{}\" at {:?}", clap_id, file_path);

    Ok(file)
}
