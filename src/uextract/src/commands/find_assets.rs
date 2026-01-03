//! Find assets by class command

use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use retoc::{
    container_header::EIoContainerHeaderVersion, iostore, zen::FZenPackageHeader, AesKey, Config,
    EIoStoreTocVersion, FGuid,
};
use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;
use std::sync::Arc;

/// Find assets by class type
#[allow(clippy::too_many_lines)]
pub fn find_assets_by_class(
    input: &Path,
    class_name: &str,
    scriptobjects_path: &Path,
    aes_key: Option<&str>,
    output: Option<&Path>,
) -> Result<()> {
    use retoc::script_objects::FPackageObjectIndexType;

    eprintln!("Searching for assets of class: {}", class_name);

    // Load scriptobjects
    let so_data = std::fs::read_to_string(scriptobjects_path)
        .with_context(|| format!("Failed to read {:?}", scriptobjects_path))?;
    let so_json: serde_json::Value = serde_json::from_str(&so_data)?;

    // Build hash→class lookup
    let hash_to_path: HashMap<String, String> = so_json
        .get("hash_to_path")
        .and_then(|v| v.as_object())
        .context("Missing hash_to_path")?
        .iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
        .collect();

    // Find the target class hash
    let target_hash: Option<String> = so_json
        .get("objects")
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter().find(|obj| {
                obj.get("name").and_then(|n| n.as_str()) == Some(class_name)
                    || obj
                        .get("path")
                        .and_then(|p| p.as_str())
                        .map(|p| p.ends_with(&format!(".{}", class_name)))
                        .unwrap_or(false)
            })
        })
        .and_then(|obj| {
            obj.get("hash")
                .and_then(|h| h.as_str())
                .map(|s| s.to_string())
        });

    let target_hash =
        target_hash.context(format!("Class '{}' not found in scriptobjects", class_name))?;
    let target_path = hash_to_path.get(&target_hash).cloned().unwrap_or_default();
    eprintln!("Target class: {} -> {}", target_hash, target_path);

    // Build retoc config
    let mut aes_keys = HashMap::new();
    if let Some(key) = aes_key {
        let parsed_key: AesKey = key.parse()?;
        aes_keys.insert(FGuid::default(), parsed_key);
    }
    let config = Arc::new(Config {
        aes_keys,
        container_header_version_override: None,
        toc_version_override: None,
    });

    // Open IoStore
    let store = iostore::open(input, config)?;

    // Get container versions
    let toc_version = store
        .container_file_version()
        .unwrap_or(EIoStoreTocVersion::ReplaceIoChunkHashWithIoHash);
    let container_header_version = store
        .container_header_version()
        .unwrap_or(EIoContainerHeaderVersion::NoExportInfo);

    // Scan all .uasset files
    let uasset_entries: Vec<_> = store
        .chunks()
        .filter_map(|chunk| {
            chunk.path().and_then(|path| {
                if path.ends_with(".uasset") {
                    Some((chunk, path))
                } else {
                    None
                }
            })
        })
        .collect();

    eprintln!("Scanning {} .uasset files...", uasset_entries.len());

    let pb = ProgressBar::new(uasset_entries.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Check each asset's class_index
    let matching_paths: Vec<String> = uasset_entries
        .par_iter()
        .filter_map(|(chunk, path)| {
            pb.inc(1);

            // Read the asset data
            let data = chunk.read().ok()?;

            // Quick parse to get export class_index
            let mut cursor = Cursor::new(&data);
            let header = FZenPackageHeader::deserialize(
                &mut cursor,
                None,
                toc_version,
                container_header_version,
                None,
            )
            .ok()?;

            // Check each export's class_index
            for export in &header.export_map {
                if export.class_index.kind() == FPackageObjectIndexType::ScriptImport {
                    let class_hash = format!("{:X}", export.class_index.raw_index());
                    if class_hash == target_hash {
                        return Some(path.clone());
                    }
                }
            }
            None
        })
        .collect();

    pb.finish_and_clear();

    eprintln!(
        "Found {} assets of class {}",
        matching_paths.len(),
        class_name
    );

    // Output results
    for path in &matching_paths {
        println!("{}", path);
    }

    // Write to file if requested
    if let Some(out_path) = output {
        let content = matching_paths.join("\n");
        std::fs::write(out_path, content)?;
        eprintln!("Wrote paths to {:?}", out_path);
    }

    Ok(())
}
