# Chapter 6: Data Extraction

A save editor needs game data: weapon stats, part definitions, manufacturer information. You might assume this data lives neatly in game files, waiting to be extracted. The reality is more complicated—and more interesting.

This chapter explores what data we can extract, what we can't, and why. Along the way, we'll document our investigation into authoritative category mappings, including the binary analysis that revealed why some data simply doesn't exist in extractable form.

---

## The Game File Landscape

BL4's data lives in Unreal Engine pak files, stored in IoStore format:

```
Borderlands 4/OakGame/Content/Paks/
├── pakchunk0-Windows_0_P.utoc    ← Main game assets
├── pakchunk0-Windows_0_P.ucas    ← Compressed data
├── pakchunk2-Windows_0_P.utoc    ← Audio (Wwise)
├── pakchunk3-Windows_0_P.utoc    ← Localized audio
├── global.utoc                   ← Shared engine data
└── ...
```

**IoStore** is UE5's container format, splitting asset indices (`.utoc`) from compressed data (`.ucas`). This differs from older PAK-only formats and requires specialized tools.

!!! note
    BL4 uses IoStore (UE5's format), not legacy PAK. Tools like `repak` won't work on `.utoc/.ucas` files. You need `retoc` or similar IoStore-aware extractors.

---

## What We Can Extract

Some game data extracts cleanly from pak files:

**Balance data**: Stat templates and modifiers for weapons, shields, and gear. These define base damage, fire rate, accuracy scales.

**Naming strategies**: How weapons get their prefix names. "Damage → Tortuous" mappings live in extractable assets.

**Body definitions**: Weapon body assets that reference parts and mesh fragments.

**Loot pools**: Drop tables and rarity weights for different sources.

**Gestalt meshes**: Visual mesh fragments that parts reference.

These assets follow Unreal's content structure:

```
OakGame/Content/
├── Gear/
│   ├── Weapons/
│   │   ├── _Shared/BalanceData/
│   │   ├── Pistols/JAK/Parts/
│   │   └── ...
│   └── Shields/
├── PlayerCharacters/
│   ├── DarkSiren/
│   └── ...
└── GameData/Loot/
```

---

## What We Can't Extract

Here's where it gets interesting. The mappings between serial tokens and actual game parts—the heart of what makes serial decoding work—don't exist as extractable pak file assets.

We wanted authoritative category mappings. Serial token `{4}` on a Vladof SMG should mean a specific part, and we wanted the game's own data to tell us which one. So we investigated.

### The Investigation: Binary Analysis

We used Rizin (a radare2 fork) to analyze the Borderlands4.exe binary directly:

```bash
rz-bin -S Borderlands4.exe
```

Results:
- Total size: 715 MB
- .sdata section: 157 MB (code)
- .rodata section: 313 MB (read-only data)

We searched for part prefix strings like "DAD_PS.part_" and "VLA_SM.part_barrel". Nothing. The prefixes don't exist as literal strings in the binary.

We searched for category value sequences. Serial decoding uses Part Group IDs like 2, 3, 4, 5, 6, 7 (consecutive integers stored as i64). We found one promising sequence at offset 0x02367554:

```python
# Found sequence 2,3,4,5,6,7 as consecutive i64 values at 0x02367554
```

But examining the context revealed it was near crypto code—specifically "Poly1305 for x86_64, CRYPTOGAMS". Those consecutive integers were coincidental, not category definitions.

!!! warning "False Positives"
    When searching binaries for numeric patterns, verify the context. Small consecutive integers appear in many places: crypto code, lookup tables, version numbers. Always examine surrounding bytes.

### UE5 Metadata: What We Know

From usmap analysis, we confirmed the exact structure linking parts to serials:

```
GbxSerialNumberIndex (12 bytes)
├── Category (Int64): Part Group ID
├── scope (Byte): EGbxSerialNumberIndexScope (Root=1, Sub=2)
├── status (Byte): EGbxSerialNumberIndexStatus
└── Index (Int16): Position in category
```

Every `InventoryPartDef` contains this structure. The `Category` field maps to Part Group IDs (2=Daedalus Pistol, 22=Vladof SMG, etc.). The `Index` field determines which part token decodes to this part.

But here's the problem: we found **zero** `InventoryPartDef` assets in pak files.

```bash
uextract /path/to/Paks find-by-class InventoryPartDef
# Result: 0 assets found
```

### Where Parts Actually Live

Parts aren't stored as individual pak file assets. They're:

1. **Runtime UObjects** — Created when the game initializes
2. **Code-defined** — Registrations happen in native code
3. **Embedded in NexusConfigStore** — Gearbox's custom data system
4. **Index-assigned at runtime** — `GbxSerialNumberIndex` values set during registration

The game's binary contains the logic to create parts, but the category-to-part mappings are computed, not stored as data files. This is why memory dumps are essential—they capture the runtime state after the game has built these structures.

!!! tip "Practical Implication"
    Don't search pak files for part definitions. Extract part data from memory dumps where the game has already assembled the complete structures.

---

## The Practical Solution: Empirical Validation

Since authoritative mappings aren't extractable, we derive them empirically:

1. Collect serials from real game items
2. Decode the Part Group ID and part tokens
3. Record which weapon/part combinations the tokens represent
4. Validate by injecting serials into saves and checking in-game

This approach produces reliable mappings. When you decode `{4}` on category 22 as a specific Vladof SMG barrel, it's because we verified that serial produces that barrel in-game.

The `part_categories.json` file in the project contains these empirically-derived mappings. They work. They just don't come from a single authoritative game file.

---

## Extraction Tools

### retoc — IoStore Extraction

The essential tool for BL4's pak format:

```bash
cargo install --git https://github.com/trumank/retoc retoc_cli

# List assets in a container
retoc list /path/to/pakchunk0-Windows_0_P.utoc

# Extract all assets
retoc unpack /path/to/pakchunk0-Windows_0_P.utoc ./output/
```

!!! warning
    For converting to legacy format, point at the **Paks directory**, not a single file. The tool needs access to `global.utoc` for ScriptObjects:
    ```bash
    retoc to-legacy /path/to/Paks/ ./output/ --no-script-objects
    ```

### uextract — Project Tool

The bl4 project's custom extraction tool:

```bash
cargo build --release -p uextract

# List all assets
./target/release/uextract /path/to/Paks --list

# Extract with filtering
./target/release/uextract /path/to/Paks -o ./output --ifilter "BalanceData"

# Use usmap for property resolution
./target/release/uextract /path/to/Paks -o ./output --usmap share/borderlands.usmap
```

---

## The Usmap Requirement

UE5 uses "unversioned" serialization. Properties are stored without field names:

```
Versioned (old):   "Damage": 50.0, "Level": 10
Unversioned (new): 0x42480000 0x0000000A
                   └── Just values, no names
```

To parse unversioned data, you need a usmap file containing the schema—all class definitions, property names, types, and offsets.

We generate usmap from memory dumps:

```bash
bl4 memory --dump share/dumps/game.dmp dump-usmap

# Output: mappings.usmap
# Names: 64917, Enums: 2986, Structs: 16849, Properties: 58793
```

The project includes a pre-generated usmap at `share/manifest/mappings.usmap`.

---

## Extracting Parts from Memory

Since parts only exist at runtime, memory extraction is the path forward.

### Step 1: Create Memory Dump

Follow Chapter 3's instructions to capture game memory while playing.

### Step 2: Extract Part Names

```bash
bl4 memory --dump share/dumps/game.dmp dump-parts \
    -o share/manifest/parts_dump.json
```

This scans for strings matching `XXX_YY.part_*` patterns:

```json
{
  "DAD_AR": [
    "DAD_AR.part_barrel_01",
    "DAD_AR.part_barrel_01_a",
    "DAD_AR.part_body"
  ],
  "VLA_SM": [
    "VLA_SM.part_barrel_01"
  ]
}
```

### Step 3: Build Parts Database

```bash
bl4 memory --dump share/dumps/game.dmp build-parts-db \
    -i share/manifest/parts_dump.json \
    -o share/manifest/parts_database.json
```

The result maps parts to categories and indices:

```json
{
  "parts": [
    {"category": 2, "index": 0, "name": "DAD_PS.part_barrel_01"},
    {"category": 22, "index": 5, "name": "VLA_SM.part_body_a"}
  ],
  "categories": {
    "2": {"count": 74, "name": "Daedalus Pistol"},
    "22": {"count": 84, "name": "Vladof SMG"}
  }
}
```

!!! important "Index Ordering"
    Part indices from memory dumps reflect the game's internal registration order—not alphabetical. Parts typically register in this order: unique variants, bodies, barrels, shields, magazines, scopes, grips, licensed parts. Alphabetical sorting produces wrong indices.

---

## Working with Extracted Assets

### Asset Structure

Extracted `.uasset` files follow the Zen package format:

```
Package
├── Header
├── Name Map (local FNames)
├── Import Map (external dependencies)
├── Export Map (objects defined here)
└── Export Data (serialized properties)
```

With usmap, these parse into readable JSON:

```json
{
  "asset_path": "OakGame/Content/Gear/Weapons/_Shared/BalanceData/WeaponStats/Struct_Weapon_Barrel_Init",
  "exports": [
    {
      "class": "ScriptStruct",
      "properties": {
        "Damage_Scale": 1.0,
        "FireRate_Scale": 1.0,
        "Accuracy_Scale": 1.0
      }
    }
  ]
}
```

### Finding Specific Data

```bash
# Find legendary items
find ./bl4_assets -name "*legendary*" -type f

# Find manufacturer data
find ./bl4_assets -iname "*manufacturer*"

# Search asset contents
grep -r "Linebacker" ./bl4_assets --include="*.uasset" -l
```

### Stat Patterns

Stats follow naming conventions: `StatName_ModifierType_Index_GUID`

| Modifier | Meaning |
|----------|---------|
| `Scale` | Multiplier (×) |
| `Add` | Flat addition (+) |
| `Value` | Absolute override |
| `Percent` | Percentage bonus |

---

## Oodle Compression

BL4 uses Oodle compression (RAD Game Tools). The `retoc` tool handles decompression automatically by loading the game's DLL:

```
~/.steam/steam/steamapps/common/"Borderlands 4"/Engine/Binaries/ThirdParty/Oodle/
└── oo2core_9_win64.dll
```

!!! tip
    If extraction fails with Oodle errors, verify the game is installed and the DLL path is accessible. On Linux, Wine must be able to load the DLL.

---

## Building a Data Pipeline

An automated extraction script saves time when the game updates:

```bash
#!/bin/bash
GAME_DIR="$HOME/.steam/steam/steamapps/common/Borderlands 4"
OUTPUT_DIR="./bl4_data"
USMAP="./share/manifest/mappings.usmap"

# Extract pak files
retoc unpack "$GAME_DIR/OakGame/Content/Paks/pakchunk0-Windows_0_P.utoc" "$OUTPUT_DIR/raw"

# Parse with usmap
./target/release/uextract "$OUTPUT_DIR/raw" -o "$OUTPUT_DIR/parsed" --usmap "$USMAP"

# Generate manifest
bl4-research pak-manifest -e "$OUTPUT_DIR/parsed" -o "$OUTPUT_DIR/manifest"
```

---

## Summary: Data Sources

| Data | Source | Extractable? |
|------|--------|--------------|
| Balance/stats | Pak files | Yes |
| Naming strategies | Pak files | Yes |
| Loot pools | Pak files | Yes |
| Body definitions | Pak files | Yes |
| Part definitions | Memory only | No |
| Category mappings | Empirical | Derived |

The gap between "what the game knows" and "what we can extract from files" is real. Memory analysis and empirical validation fill that gap.

---

## Exercises

**Exercise 1: Extract and Explore**

Extract the main pak file. Find balance data for a weapon type you use. What stats does the base template define?

**Exercise 2: Search for Part References**

Search extracted assets for references to specific parts (like "JAK_PS.part_barrel"). Where do they appear? What references them?

**Exercise 3: Compare Manufacturers**

Extract assets for two manufacturers (Jakobs vs Maliwan). Compare directory structures. What patterns emerge?

---

## What's Next

We've covered the full data extraction story—what works, what doesn't, and why. The bl4 project wraps all these techniques into command-line tools.

Next, we'll tour those tools: how to decode serials, edit saves, extract data, and more, all from the command line.

**Next: [Chapter 7: Using bl4 Tools](07-bl4-tools.md)**
