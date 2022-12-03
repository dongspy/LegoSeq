#[macro_use]
extern crate lazy_static;
use anyhow::{Error, Ok, Result};
use bio::io::fasta;
use bio::io::fastq::{self, FastqRead, Reader, Record};
use clap::builder::Str;
use crossbeam_channel::{unbounded, Receiver, Sender};
use rayon;
use serde::Deserialize;
use std::hash::Hash;
use std::{collections::HashMap, fmt, str};
use std::{f32::MAX, fs::File, io::BufReader};
use utils::{get_fasta_reader, get_fastq_reader};

mod sequence;
pub mod utils;

// id,fq_file,seq_type,start,end,barcode_file,anchor_seq

#[derive(Debug, Deserialize)]
struct Config {
    id: String,
    fq_file: String,
    seq_type: String,
    start: isize,
    end: isize,
    barcode_file: Option<String>,
    // anchor_seq: Option<String>
    anchor_seq: String,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.id, self.seq_type, self.anchor_seq)
    }
}

/// read the csv file of configure to HashMap
fn get_config(path: &str) -> Result<HashMap<String, Vec<Config>>> {
    // let path = "test/config.csv";
    let mut fq_hash: HashMap<String, Vec<Config>> = HashMap::new();
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(buffered);
    for record in reader.deserialize() {
        let record: Config = record?;
        // dbg!("{:#?}", record);
        // fq_hash.insert((&record).fq_file.to_owned(), record);
        fq_hash
            .entry((&record).fq_file.to_owned())
            .or_insert(Vec::new())
            .push(record);
    }
    Ok(fq_hash)
}

pub struct FastqRecordSet {
    reader_hash: HashMap<String, Box<dyn FastqRead>>,
}

impl FastqRecordSet {
    fn next(&mut self) -> Option<Result<HashMap<String, Record>>> {
        let mut record_hash: HashMap<String, Record> = HashMap::new();
        for (file, reader) in self.reader_hash.iter_mut() {
            let mut record = Record::new();
            reader.read(&mut record).unwrap();
            if record.is_empty() {
                return None;
            }
            record_hash.insert(file.to_owned(), record);
        }

        Some(Ok(record_hash))
    }
}

/// get the barcode sequence
fn get_barcode(path: &str) -> Result<HashMap<String, Vec<u8>>> {
    let mut barcode_hash: HashMap<String, Vec<u8>> = HashMap::new();
    let mut fasta_reader = get_fasta_reader(path);
    loop {
        let mut record = fasta::Record::new();
        fasta_reader.read(&mut record)?;
        if record.is_empty() {
            break;
        }
        barcode_hash.insert(record.id().to_owned(), record.seq().to_owned());
    }

    Ok(barcode_hash)
}

// fn get_seq_from_config(record:fastq::Record, config:Config) -> Result<()>{

// }

fn main() -> Result<()> {
    let mut config_hash = get_config("test/config.csv").unwrap();
    let mut record_set = FastqRecordSet {
        reader_hash: config_hash
            .iter()
            .map(|(x, y)| (x.to_owned(), get_fastq_reader(x)))
            .collect::<HashMap<String, Box<dyn FastqRead>>>(),
    };

    while let Some(rs) = record_set.next() {
        let record_hash = rs.unwrap();
        for (file, record) in record_hash.iter() {
            println!("file: {}; read:{}", file, record.id());
            // let subseq = config_hash.entry(key);
            let config = config_hash.get(file).unwrap();
            let seq_type_str = String::from("placeholder");
            // dbg!(config);
            let test = config
                .iter()
                .map(|x| match &x.seq_type {
                    seq_type_str => &x.seq_type,
                    _ => &x.seq_type,
                })
                .collect::<Vec<&String>>();
            dbg!(test);
        }
    }

    Ok(())
}
