//! NCS extract command

use anyhow::{Context, Result};
use bl4_ncs::NcsContent;
use std::fs;
use std::path::{Path, PathBuf};

use super::types::{FileInfo, PartIndex};

/// Known weapon manufacturers
const MANUFACTURERS: &[&str] = &["BOR", "DAD", "JAK", "MAL", "ORD", "TED", "TOR", "VLA"];

/// Known weapon types
const WEAPON_TYPES: &[&str] = &["AR", "PS", "SG", "SM", "SR"];

pub fn extract_by_type(
    path: &Path,
    extract_type: &str,
    output: Option<&Path>,
    json: bool,
) -> Result<()> {
    // Special handling for "parts" extraction
    if extract_type == "parts" {
        return extract_part_indices(path, output, json);
    }

    let mut extracted = Vec::new();

    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path();
        if !file_path.extension().map(|e| e == "bin").unwrap_or(false) {
            continue;
        }

        if let Ok(data) = fs::read(file_path) {
            if let Some(content) = NcsContent::parse(&data) {
                if content.type_name() == extract_type {
                    extracted.push(FileInfo {
                        path: file_path.to_string_lossy().to_string(),
                        type_name: content.type_name().to_string(),
                        format_code: content.format_code().to_string(),
                        entry_names: content.entry_names().map(|s| s.to_string()).collect(),
                        guids: content.guids().map(|s| s.to_string()).collect(),
                        numeric_values: content
                            .numeric_values()
                            .map(|(s, v)| (s.to_string(), v))
                            .collect(),
                    });
                }
            }
        }
    }

    let output_str = if json {
        serde_json::to_string_pretty(&extracted)?
    } else {
        let mut out = format!("=== Extracted {} entries ===\n\n", extracted.len());
        for info in &extracted {
            out.push_str(&format!("File: {}\n", info.path));
            out.push_str(&format!("Format: {}\n", info.format_code));
            out.push_str("Entries:\n");
            for name in &info.entry_names {
                out.push_str(&format!("  - {}\n", name));
            }
            out.push('\n');
        }
        out
    };

    if let Some(output_path) = output {
        fs::write(output_path, &output_str)?;
        println!(
            "Wrote {} entries to {}",
            extracted.len(),
            output_path.display()
        );
    } else {
        println!("{}", output_str);
    }

    Ok(())
}

/// Extract part serial indices from inv.bin
///
/// The inv.bin NCS file contains part definitions where:
/// - Part names follow pattern: MANU_TYPE_PartName (e.g., BOR_SG_Grip_01)
/// - Serial index immediately follows as a decimal string
fn extract_part_indices(path: &Path, output: Option<&Path>, json: bool) -> Result<()> {
    // Find inv.bin file
    let inv_path = find_inv_bin(path)?;
    let data = fs::read(&inv_path).context("Failed to read inv.bin")?;

    // Extract null-terminated strings
    let strings = extract_null_strings(&data);

    let mut parts = Vec::new();

    for i in 0..strings.len().saturating_sub(1) {
        let s = &strings[i];

        // Check if this looks like a part name (MANU_TYPE_Name pattern)
        if let Some((manufacturer, weapon_type)) = parse_part_name(s) {
            // Look for numeric index within next 10 strings (indices often have fields between)
            let window_end = (i + 10).min(strings.len());
            for j in (i + 1)..window_end {
                let candidate = &strings[j];

                // Stop if we hit another part name (new record)
                if parse_part_name(candidate).is_some() {
                    break;
                }

                // Check if this is a small integer (serial indices are typically < 1000)
                if let Ok(idx) = candidate.parse::<u32>() {
                    if idx < 1000 {
                        parts.push(PartIndex {
                            part_name: s.clone(),
                            serial_index: idx,
                            manufacturer,
                            weapon_type,
                        });
                        break;
                    }
                }
            }
        }
    }

    // Sort by manufacturer, weapon type, then index
    parts.sort_by(|a, b| {
        (&a.manufacturer, &a.weapon_type, a.serial_index)
            .cmp(&(&b.manufacturer, &b.weapon_type, b.serial_index))
    });

    let output_str = if json {
        serde_json::to_string_pretty(&parts)?
    } else {
        // TSV output
        let mut out = String::from("part_name\tserial_index\tmanufacturer\tweapon_type\n");
        for p in &parts {
            out.push_str(&format!(
                "{}\t{}\t{}\t{}\n",
                p.part_name, p.serial_index, p.manufacturer, p.weapon_type
            ));
        }
        out
    };

    if let Some(output_path) = output {
        fs::write(output_path, &output_str)?;
        println!(
            "Extracted {} part indices to {}",
            parts.len(),
            output_path.display()
        );
    } else {
        print!("{}", output_str);
    }

    eprintln!("\n# Total: {} parts with serial indices", parts.len());

    Ok(())
}

/// Find inv.bin file in a directory
fn find_inv_bin(path: &Path) -> Result<PathBuf> {
    // If path is a file, use it directly
    if path.is_file() {
        return Ok(path.to_path_buf());
    }

    // Search for inv.bin in directory
    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path();
        let name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if name == "inv.bin" {
            return Ok(file_path.to_path_buf());
        }
    }

    anyhow::bail!("inv.bin not found in {}", path.display())
}

/// Extract null-terminated strings from binary data
fn extract_null_strings(data: &[u8]) -> Vec<String> {
    let mut strings = Vec::new();
    let mut current = Vec::new();

    for &b in data {
        if b == 0 {
            if !current.is_empty() {
                if let Ok(s) = std::str::from_utf8(&current) {
                    if !s.is_empty() {
                        strings.push(s.to_string());
                    }
                }
                current.clear();
            }
        } else if (32..=126).contains(&b) {
            current.push(b);
        } else {
            current.clear();
        }
    }

    strings
}

/// Parse a part name in MANU_TYPE_Name format
/// Returns (manufacturer, weapon_type) if valid, None otherwise
fn parse_part_name(s: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = s.splitn(3, '_').collect();
    if parts.len() < 3 {
        return None;
    }

    let manufacturer = parts[0];
    let weapon_type = parts[1];

    // Must be a known manufacturer
    if !MANUFACTURERS.contains(&manufacturer) {
        return None;
    }

    // Must be a known weapon type
    if !WEAPON_TYPES.contains(&weapon_type) {
        return None;
    }

    // Rest of the name must be alphanumeric with underscores
    let rest = parts[2];
    if rest.is_empty() || !rest.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }

    Some((manufacturer.to_string(), weapon_type.to_string()))
}
