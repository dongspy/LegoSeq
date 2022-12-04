use std::collections::HashMap;

use anyhow::Ok;
use bio::io::fastq;

use crate::utils::get_read_best_index_local;
use crate::Config;

#[derive(Debug)]
struct PlaceHolderSeq {
    id: u8,
    config_start: isize,
    config_end: isize,
    real_start: usize,
    real_end: usize,
    seq: String,
    qual: String,
}

impl PlaceHolderSeq {
    fn new(config: Config, record: &fastq::Record) -> Self {
        PlaceHolderSeq {
            id: config.id,
            config_start: config.start,
            config_end: config.end,
            real_start: config.start as usize,
            real_end: config.end as usize,
            seq: String::from_utf8(
                record.seq()[config.start as usize..config.end as usize].to_vec(),
            )
            .unwrap(), //[config.start..config.end],
            qual: String::from_utf8(
                record.qual()[config.start as usize..config.end as usize].to_vec(),
            )
            .unwrap(),
        }
    }
}

#[derive(Debug)]
struct BarcodeSeq {
    id: u8,
    config_start: isize,
    config_end: isize,
    real_start: usize,
    real_end: usize,
    seq: String,
    qual: String,
    barcode_seq: HashMap<String, Vec<u8>>,
    best_barcode: String,
}

impl BarcodeSeq {
    pub fn new(config: Config, record: &fastq::Record) -> Self {
        BarcodeSeq {
            id: config.id,
            config_start: config.start,
            config_end: config.end,
            real_start: config.start as usize,
            real_end: config.end as usize,
            seq: String::new(),  //[config.start..config.end],
            qual: String::new(), //String::from_utf8(record.qual()[config.start as usize..config.end as usize].to_vec()).unwrap(),
            barcode_seq: HashMap::new(),
            best_barcode: String::new(),
        }
    }

    pub fn get_best_index(
        &self,
        record: &fastq::Record,
        barcode_hash: &HashMap<String, HashMap<u8, HashMap<String, Vec<u8>>>>,
        max_mismatch: usize,
    ) -> Result<()> {
        let index = get_read_best_index_local(barcode_hash, record.seq(), max_mismatch);
        Ok(())
    }
}

#[derive(Debug)]
struct AnchorSeq {
    id: u8,
    config_start: isize,
    config_end: isize,
    real_start: usize,
    real_end: usize,
    seq: Vec<String>,
    qual: Vec<String>,
    anchor_seq: HashMap<String, Vec<u8>>,
}
