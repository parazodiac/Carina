use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io::Write;
use std::path::Path;
use std::collections::HashSet;

use clap::ArgMatches;

use rust_htslib::bam;
use rust_htslib::bam::{Read, Format};

pub fn callback(sub_m: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let bam_file_path = Path::new(sub_m.value_of("ibam").expect("can't find BAM flag"))
        .canonicalize()
        .expect("can't find absolute path of input file");

    info!("Found BAM files: {:?}", bam_file_path);
    let mut input_bam = bam::Reader::from_path(bam_file_path.clone())
        .expect("Can't open BAM file");
    input_bam.set_threads(3).unwrap();
    let header = bam::Header::from_template(input_bam.header());

    let subsampled_file_path = bam_file_path
        .parent()
        .unwrap()
        .join(bam_file_path.file_stem().unwrap())
        .to_str()
        .unwrap()
        .to_owned()
        + ".subsampled.bam";
    info!("Creating BAM files: {:?}", subsampled_file_path);
    let mut output_bam = bam::Writer::from_path(subsampled_file_path, 
                                                &header,
                                                Format::BAM)
        .expect("Can't open BAM file");
    output_bam.set_threads(3).unwrap();


    let cb_file_path = sub_m.value_of("icb").expect("can't find tsv flag");
    let cb_file = BufReader::new(File::open(cb_file_path)?);
    let cb_file_path = Path::new(cb_file_path)
        .canonicalize()
        .expect("can't resolved absolute cb file path");
    info!("Found CB file: {:?}", cb_file_path);

    let mut cb_list: HashSet<u64> = HashSet::new();
    for line in cb_file.lines() {
        let line = line.unwrap();
        let cb = crate::utils::cb_string_to_u64(line.as_bytes())
            .unwrap();
        cb_list.insert(cb);
    }

    let mut total_lines = 0;
    let mut total_written = 0;
    for alignment in input_bam.records()
        .into_iter()
    {
        total_lines += 1;
        if total_lines % crate::MIL == 0 {
            print!(
                "\rDone processing {}M lines",
                total_lines / crate::MIL
            );
            std::io::stdout().flush().expect("Can't flush output");
        }

        let alignment = alignment.unwrap();
        if let Some(some_cb) = alignment.aux(b"CB") {
            let cb: u64 = crate::utils::cb_string_to_u64(&some_cb.string()[..crate::CB_LENGTH])
                .expect("can't convert cb string to u64");

            if cb_list.contains(&cb){
                total_written += 1;
                output_bam.write(&alignment).unwrap();
            }
        }
    } // end iterating bam

    println!("Saw total / wrote total {}/{} lines", total_lines, total_written);

    Ok(())
}