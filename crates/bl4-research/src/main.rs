//! UAsset parser for researching BL4 weapon part data
//!
//! Usage: cargo run -p bl4-research -- <file.uasset>

use std::env;
use std::io::Cursor;
use unreal_asset::engine_version::EngineVersion;
use unreal_asset::exports::ExportBaseTrait;
use unreal_asset::Asset;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file.uasset>", args[0]);
        eprintln!("  Parses a UE5 .uasset file and dumps export info");
        return;
    }

    let path = &args[1];
    println!("Parsing: {}", path);

    // Read file into memory
    let data = match std::fs::read(path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };

    // Try different engine versions (UE4 versions since UE5 might use same format)
    let versions = [
        EngineVersion::VER_UE4_27,
        EngineVersion::VER_UE4_26,
        EngineVersion::VER_UE4_25,
        EngineVersion::UNKNOWN,
    ];

    for version in versions {
        let cursor = Cursor::new(&data);
        match Asset::new(cursor, None, version) {
            Ok(asset) => {
                println!("Successfully parsed with {:?}", version);
                println!("Exports: {}", asset.asset_data.exports.len());

                for (i, export) in asset.asset_data.exports.iter().enumerate() {
                    let base = export.get_base_export();
                    println!(
                        "  [{}] {:?} (class: {:?})",
                        i, base.object_name, base.class_index
                    );
                }
                return;
            }
            Err(e) => {
                println!("Failed with {:?}: {}", version, e);
                continue;
            }
        }
    }

    eprintln!("Failed to parse with any engine version");
}
