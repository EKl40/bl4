use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

pub fn read_records(path: &Path, format: &str) -> Result<Vec<Vec<u8>>> {
    let file = File::open(path).with_context(|| format!("Failed to open {:?}", path))?;

    match format {
        "length16" => {
            let mut reader = BufReader::new(file);
            let mut records = Vec::new();

            loop {
                let mut len_buf = [0u8; 2];
                match reader.read_exact(&mut len_buf) {
                    Ok(()) => {}
                    Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                    Err(e) => return Err(e.into()),
                }

                let len = u16::from_le_bytes(len_buf) as usize;
                if len == 0 {
                    records.push(Vec::new());
                    continue;
                }

                let mut data = vec![0u8; len];
                reader.read_exact(&mut data)?;
                records.push(data);
            }

            Ok(records)
        }
        "lines" => {
            let reader = BufReader::new(file);
            let mut records = Vec::new();

            for line in reader.lines() {
                let line = line?;
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                let bytes: Result<Vec<u8>, _> = (0..line.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&line[i..i + 2], 16))
                    .collect();

                records.push(bytes.context("Invalid hex")?);
            }

            Ok(records)
        }
        _ => anyhow::bail!("Unknown format: {}", format),
    }
}

pub fn group_by_position(records: &[Vec<u8>], position: usize) -> HashMap<u8, Vec<&Vec<u8>>> {
    let mut groups: HashMap<u8, Vec<&Vec<u8>>> = HashMap::new();

    for record in records {
        if let Some(&byte) = record.get(position) {
            groups.entry(byte).or_default().push(record);
        }
    }

    groups
}

pub fn filter_by_position(records: &[Vec<u8>], position: usize, value: u8) -> Vec<&Vec<u8>> {
    records
        .iter()
        .filter(|r| r.get(position) == Some(&value))
        .collect()
}
