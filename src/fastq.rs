use bio::io::fastq::{Reader, Record, Records};

use std::fs::File;
use std::path::PathBuf;

// two fastq file experiment
pub struct FastqFeeder2<R: std::io::Read> {
    fastq_one: Records<R>,
    fastq_two: Records<R>,
}

impl<R: std::io::Read> FastqFeeder2<R> {
    pub fn new(fastq_paths: Vec<PathBuf>) -> FastqFeeder2<File> {
        assert!(fastq_paths.len() == 2, "found more than two files");

        let fastq_one = Reader::from_file(fastq_paths.first().unwrap())
            .expect("can't open fastq file 1")
            .records();
        let fastq_two = Reader::from_file(fastq_paths.last().unwrap())
            .expect("can't open fastq file 2")
            .records();

        FastqFeeder2 {
            fastq_one,
            fastq_two,
        }
    }
}

impl<R: std::io::Read> Iterator for FastqFeeder2<R> {
    type Item = (Record, Record);
    fn next(&mut self) -> Option<Self::Item> {
        let rec_one = match self.fastq_one.next() {
            Some(Ok(rec)) => rec,
            _ => return None,
        };

        let rec_two = match self.fastq_two.next() {
            Some(Ok(rec)) => rec,
            _ => return None,
        };

        Some((rec_one, rec_two))
    }
}

// three fastq file experiment
pub struct FastqFeeder3<R: std::io::Read> {
    fastq_one: Records<R>,
    fastq_two: Records<R>,
    fastq_three: Records<R>,
}

impl<R: std::io::Read> FastqFeeder3<R> {
    pub fn new(fastq_paths: Vec<PathBuf>) -> FastqFeeder3<File> {
        assert!(fastq_paths.len() == 3, "found more than three files");

        let fastq_one = Reader::from_file(fastq_paths[0].as_path())
            .expect("can't open fastq file 1")
            .records();
        let fastq_two = Reader::from_file(fastq_paths[1].as_path())
            .expect("can't open fastq file 2")
            .records();
        let fastq_three = Reader::from_file(fastq_paths[2].as_path())
            .expect("can't open fastq file 3")
            .records();

        FastqFeeder3 {
            fastq_one,
            fastq_two,
            fastq_three,
        }
    }
}

impl<R: std::io::Read> Iterator for FastqFeeder3<R> {
    type Item = (Record, Record, Record);
    fn next(&mut self) -> Option<Self::Item> {
        let rec_one = match self.fastq_one.next() {
            Some(Ok(rec)) => rec,
            _ => return None,
        };

        let rec_two = match self.fastq_two.next() {
            Some(Ok(rec)) => rec,
            _ => return None,
        };

        let rec_three = match self.fastq_three.next() {
            Some(Ok(rec)) => rec,
            _ => return None,
        };

        Some((rec_one, rec_two, rec_three))
    }
}
