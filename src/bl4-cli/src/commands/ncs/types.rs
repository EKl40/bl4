//! NCS command type definitions

use serde::Serialize;
use std::collections::HashMap;

/// Result of scanning a directory
#[derive(Debug, Serialize)]
pub struct ScanResult {
    pub total_files: usize,
    pub parsed_files: usize,
    pub types: HashMap<String, Vec<String>>,
    pub formats: HashMap<String, usize>,
}

/// Information about a single NCS file
#[derive(Debug, Serialize)]
pub struct FileInfo {
    pub path: String,
    pub type_name: String,
    pub format_code: String,
    pub entry_names: Vec<String>,
    pub guids: Vec<String>,
    pub numeric_values: Vec<(String, f64)>,
}

/// Search result
#[derive(Debug, Serialize)]
pub struct SearchMatch {
    pub path: String,
    pub type_name: String,
    pub matches: Vec<String>,
}

/// Part index entry extracted from inv.bin
#[derive(Debug, Serialize)]
pub struct PartIndex {
    pub part_name: String,
    pub serial_index: u32,
    pub manufacturer: String,
    pub weapon_type: String,
}
