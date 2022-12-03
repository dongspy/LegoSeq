use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{self, BufWriter};
use std::str::from_utf8;
use std::{fs::File, path::Path};

use bio::alignment::pairwise::banded::Aligner;
use bio::alignment::{Alignment, AlignmentOperation};
use bio::io::{fasta, fastq};
use bio_types::alignment::AlignmentOperation::*;
use flate2::read::MultiGzDecoder;
use serde::Serialize;

pub fn get_fastq_reader(path: &str) -> Box<dyn fastq::FastqRead> {
    // let mut record = fastq::Record::new();
    let f = File::open(path).unwrap();
    let reader: Box<dyn fastq::FastqRead> = match path.ends_with(".gz") {
        true => Box::new(fastq::Reader::new(io::BufReader::new(MultiGzDecoder::new(
            f,
        )))),
        false => Box::new(fastq::Reader::new(f)),
    };
    reader
}

pub fn get_fasta_reader(path: &str) -> Box<dyn fasta::FastaRead> {
    // let mut record = fastq::Record::new();
    let f = File::open(path).unwrap();
    let reader: Box<dyn fasta::FastaRead> = match path.ends_with(".gz") {
        true => Box::new(fasta::Reader::new(io::BufReader::new(MultiGzDecoder::new(
            f,
        )))),
        false => Box::new(fasta::Reader::new(f)),
    };
    reader
}

fn banded_local_align(seq1: &[u8], seq2: &[u8]) -> (usize, usize, usize, String) {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let k = 5; // kmer match length
    let w = 4; // Window size for creating the band
    let mut aligner = Aligner::new(-2, -1, score, k, w);
    let alignment = aligner.local(seq1, seq2);
    let match_count: Vec<AlignmentOperation> = alignment
        .operations
        .into_iter()
        .filter(|x| *x == Match)
        .collect();
    let align_seq = &seq2[alignment.ystart..alignment.yend];
    return (
        match_count.len(),
        alignment.ystart,
        alignment.yend,
        from_utf8(align_seq).unwrap().to_owned(),
    );
}

#[test]
fn test_local_align() {
    let x = b"AGCACACGTGTGCGCTATACAGTAAGTAGTAGTACACGTGTCACAGTTGTACTAGCATGAC";
    let y = b"TACAGAAAGTAGT";
    let match_len = banded_local_align(x, y);
    dbg!(match_len);
}

#[derive(Debug)]
struct ReadInfo {
    read_name: String,
    best_index: String,
    align_start: usize,
    align_end: usize,
    mismatch_num: usize,
    match_seq: String,
}

impl ReadInfo {
    fn write_line(&self, shift: usize) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}",
            self.read_name,
            self.best_index,
            self.align_start + shift,
            self.align_end + shift,
            self.mismatch_num
        )
    }
}

fn get_read_best_index_local(
    index_hash: &HashMap<String, Vec<u8>>,
    seq: &[u8],
    max_mismatch: usize,
) -> String {
    let mut best_index = String::from("unknown");
    let mut max_dist = max_mismatch;
    for (sample, index_seq) in index_hash.into_iter() {
        let align_out = banded_local_align(&index_seq, seq);
        let align_start = align_out.1;
        let align_end = align_out.2;
        let seq_len = seq.len();
        if align_end + 10 > seq_len {
            continue;
        }

        let mismatch_len = index_seq.len() - align_out.0;
        if (mismatch_len < max_dist) & (seq[align_end..(align_end + 10)] == b"TTT"[..]) {
            max_dist = mismatch_len;
            best_index = sample.clone();
        }
    }
    best_index
}
