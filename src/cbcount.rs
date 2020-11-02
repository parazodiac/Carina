use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::collections::HashMap;

use clap::ArgMatches;
use itertools::Itertools;

use rust_htslib::bam;
use rust_htslib::bam::{Read, Record};

pub fn callback(sub_m: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let bam_file_path = Path::new(sub_m.value_of("ibam").expect("can't find BAM flag"))
        .canonicalize()
        .expect("can't find absolute path of input file");

    info!("Found BAM files: {:?}", bam_file_path);
    let mut input_bam = bam::Reader::from_path(bam_file_path).expect("Can't open BAM file");
    input_bam.set_threads(4).unwrap();

    let tsv_file_path = sub_m.value_of("otsv").expect("can't find tsv flag");
    let mut otsv_file = BufWriter::new(File::create(tsv_file_path)?);
    let tsv_file_path = Path::new(tsv_file_path)
        .canonicalize()
        .expect("can't resolved absolute tsv file path");
    info!("Created BED file: {:?}", tsv_file_path);

    //let (is_tenx, cb_extractor): (bool, fn(&Record) -> u64) = match sub_m.occurrences_of("tenx") {
    //    0 => unimplemented!(),
    //    _ => (true, |aln: &Record| -> u64 {
    //        crate::utils::cb_string_to_u64(&aln.aux(b"CB").unwrap().string()[..crate::CB_LENGTH])
    //            .expect("can't convert cb string to u64")
    //    }),
    //};

    let mut total_reads = 0;
    let mut cb_freq: HashMap<u64, u64> = HashMap::new() ;
    for (_, read_group) in input_bam
        .records()
        .map(|res| res.unwrap())
        .group_by(|rec| rec.qname().to_owned())
        .into_iter()
    {
        total_reads += 1;
        if total_reads % crate::MIL == 0 {
            print!(
                "\rDone processing {}M reads",
                total_reads / crate::MIL
            );
            std::io::stdout().flush().expect("Can't flush output");
        }

        let alignments: Vec<Record> = read_group.collect();
        let first_alignment = alignments.first().unwrap();

        if let Some(some_cb) = first_alignment.aux(b"CB") {
            let cb: u64 = crate::utils::cb_string_to_u64(&some_cb.string()[..crate::CB_LENGTH])
                .expect("can't convert cb string to u64");

            let val = cb_freq.entry(cb)
                .or_insert(0);
            *val += 1;
        }
    } // end iterating bam
    println!("Saw total {} reads", total_reads);

    for (key, val) in cb_freq {
        writeln!(
            &mut otsv_file,
            "{}\t{}",
            crate::utils::u64_to_cb_string(key)?,
            val
        )?;
    }

    Ok(())
}