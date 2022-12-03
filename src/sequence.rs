use std::collections::HashMap;

#[derive(Debug)]
struct PlaceHolderSeq {
    id: u8,
    config_start: isize,
    config_end: isize,
    real_start: usize,
    real_end: usize,
    seq: Vec<String>,
    qual: Vec<String>,
}

#[derive(Debug)]
struct BarcodeSeq {
    id: u8,
    config_start: isize,
    config_end: isize,
    real_start: usize,
    real_end: usize,
    seq: Vec<String>,
    qual: Vec<String>,
    barcode_seq: HashMap<String, Vec<u8>>,
    best_barcode: String,
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
