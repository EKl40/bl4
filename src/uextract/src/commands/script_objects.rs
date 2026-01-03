//! ScriptObjects extraction command

use anyhow::{Context, Result};
use retoc::{iostore, AesKey, Config, FGuid};
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

/// Entry in the ScriptObjects lookup table
#[derive(Debug, Serialize)]
pub struct ScriptObjectEntry {
    /// Object name (class name like "InventoryPartDef")
    pub name: String,
    /// Full object path (like "/Script/GbxInventory.InventoryPartDef")
    pub path: String,
    /// The hash used in FPackageObjectIndex::ScriptImport
    pub hash: String,
    /// Raw hash value as u64
    pub hash_value: u64,
    /// Outer object hash (parent)
    pub outer_hash: Option<String>,
    /// CDO class hash
    pub cdo_class_hash: Option<String>,
}

/// Full ScriptObjects dump
#[derive(Debug, Serialize)]
pub struct ScriptObjectsDump {
    /// Total count
    pub count: usize,
    /// All script objects with their hashes
    pub objects: Vec<ScriptObjectEntry>,
    /// Hash to path lookup (for quick resolution)
    pub hash_to_path: HashMap<String, String>,
}

/// Extract script objects from pak files to JSON
#[allow(clippy::too_many_lines)]
pub fn extract_script_objects(input: &Path, output: &Path, aes_key: Option<&str>) -> Result<()> {
    use retoc::script_objects::FPackageObjectIndexType;

    eprintln!("Loading ScriptObjects from {:?}", input);

    // Build retoc config
    let mut aes_keys = HashMap::new();
    if let Some(key) = aes_key {
        let parsed_key: AesKey = key
            .parse()
            .context("Invalid AES key format (use hex or base64)")?;
        aes_keys.insert(FGuid::default(), parsed_key);
    }
    let config = Arc::new(Config {
        aes_keys,
        container_header_version_override: None,
        toc_version_override: None,
    });

    // Open IoStore
    let store =
        iostore::open(input, config).with_context(|| format!("Failed to open {:?}", input))?;

    // Load ScriptObjects
    let script_objects = store
        .load_script_objects()
        .context("Failed to load ScriptObjects (is this the Paks directory with global.utoc?)")?;

    eprintln!(
        "Found {} script objects",
        script_objects.script_objects.len()
    );

    // Build the entries
    let mut objects = Vec::new();
    let mut hash_to_path = HashMap::new();

    for obj in &script_objects.script_objects {
        let name = script_objects
            .global_name_map
            .get(obj.object_name)
            .to_string();

        // Build the full path by resolving outer chain
        let path = resolve_script_object_path(obj, &script_objects);

        // Get the hash from global_index
        let hash_value = obj.global_index.raw_index();
        let hash = format!("{:X}", hash_value);

        // Get outer and cdo hashes
        let outer_hash = if obj.outer_index.kind() == FPackageObjectIndexType::ScriptImport {
            Some(format!("{:X}", obj.outer_index.raw_index()))
        } else {
            None
        };

        let cdo_class_hash = if obj.cdo_class_index.kind() == FPackageObjectIndexType::ScriptImport
        {
            Some(format!("{:X}", obj.cdo_class_index.raw_index()))
        } else {
            None
        };

        hash_to_path.insert(hash.clone(), path.clone());

        objects.push(ScriptObjectEntry {
            name,
            path,
            hash,
            hash_value,
            outer_hash,
            cdo_class_hash,
        });
    }

    let dump = ScriptObjectsDump {
        count: objects.len(),
        objects,
        hash_to_path,
    };

    // Write to JSON
    let json = serde_json::to_string_pretty(&dump)?;
    std::fs::write(output, &json).with_context(|| format!("Failed to write {:?}", output))?;

    eprintln!("Wrote {} script objects to {:?}", dump.count, output);

    // Print some stats
    let inventory_parts: Vec<_> = dump
        .objects
        .iter()
        .filter(|o| o.name.contains("InventoryPart") || o.name.contains("PartDef"))
        .collect();
    if !inventory_parts.is_empty() {
        eprintln!("\nInventoryPart-related objects:");
        for obj in inventory_parts.iter().take(10) {
            eprintln!("  {} -> {}", obj.hash, obj.path);
        }
        if inventory_parts.len() > 10 {
            eprintln!("  ... and {} more", inventory_parts.len() - 10);
        }
    }

    Ok(())
}

/// Resolve the full path of a script object by walking the outer chain
fn resolve_script_object_path(
    obj: &retoc::script_objects::FScriptObjectEntry,
    script_objects: &retoc::script_objects::ZenScriptObjects,
) -> String {
    use retoc::script_objects::FPackageObjectIndexType;

    let name = script_objects
        .global_name_map
        .get(obj.object_name)
        .to_string();

    // If no outer, this is a top-level package
    if obj.outer_index.kind() != FPackageObjectIndexType::ScriptImport {
        return name;
    }

    // Look up the outer object
    if let Some(outer) = script_objects.script_object_lookup.get(&obj.outer_index) {
        let outer_path = resolve_script_object_path(outer, script_objects);
        format!("{}.{}", outer_path, name)
    } else {
        // Outer not found, just return the name
        name
    }
}
