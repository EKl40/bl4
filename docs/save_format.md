# Borderlands 4 Save File Format

Documentation of the .sav file format used by Borderlands 4.

## File Location

Save files are typically located at:
- **Windows**: `%USERPROFILE%\Documents\My Games\Borderlands 4\Saved\SaveGames\<SteamID>\Profiles\`
- **Linux (Proton)**: `~/.local/share/Steam/steamapps/compatdata/1285190/pfx/drive_c/users/steamuser/Documents/My Games/Borderlands 4/Saved/SaveGames/<SteamID>/Profiles/`

### Directory Structure

```
Profiles/
├── 1.sav              # Profile-level data
└── client/
    ├── profile.sav    # Client profile
    ├── 1.sav          # Character slot 1
    ├── 2.sav          # Character slot 2
    ├── 3.sav          # Character slot 3
    └── 4.sav          # Character slot 4
```

Character data (inventory, equipped items, etc.) is in `Profiles/client/<N>.sav`.

## Encryption Layer

### Algorithm
- **Cipher**: AES-256-ECB
- **Padding**: PKCS7

### Key Derivation

The encryption key is derived from the Steam ID:

```rust
const BASE_KEY: [u8; 32] = [
    0x35, 0xEC, 0x33, 0x77, 0xF3, 0x5D, 0xB0, 0xEA,
    0xBE, 0x6B, 0x83, 0x11, 0x54, 0x03, 0xEB, 0xFB,
    0x27, 0x25, 0x64, 0x2E, 0xD5, 0x49, 0x06, 0x29,
    0x05, 0x78, 0xBD, 0x60, 0xBA, 0x4A, 0xA7, 0x87,
];

fn derive_key(steam_id: &str) -> [u8; 32] {
    // 1. Extract only digit characters from steam_id
    let digits: String = steam_id.chars().filter(|c| c.is_ascii_digit()).collect();

    // 2. Parse as u64 and convert to little-endian bytes
    let id_num: u64 = digits.parse().unwrap();
    let id_bytes = id_num.to_le_bytes();  // 8 bytes

    // 3. XOR first 8 bytes of BASE_KEY with steam_id bytes
    let mut key = BASE_KEY;
    for i in 0..8 {
        key[i] ^= id_bytes[i];
    }

    // 4. Return the modified key directly (no hashing)
    key
}
```

## Compression Layer

After decryption, data is zlib compressed:
- **Algorithm**: zlib (deflate)
- **Header**: Standard zlib header (0x78 0x9C for default compression)

## Data Layer (YAML)

The decompressed data is YAML formatted.

### Root Structure

```yaml
state:
  char_name: "PlayerName"
  class: "ClassName"
  player_difficulty: "Normal"
  currencies:
    cash: 1000000
    eridium: 5000
  experience:
    - type: Character
      level: 50
      points: 12345678
    - type: Specialization
      level: 25
      points: 500000
  inventory:
    items:
      backpack:
        slot_0:
          serial: "@Ugr..."
          flags: 0
        # ... more slots
  # ... additional fields
```

### Key Paths

| Path | Type | Description |
|------|------|-------------|
| `state.char_name` | string | Character name |
| `state.class` | string | Character class |
| `state.player_difficulty` | string | Difficulty setting |
| `state.currencies.cash` | u64 | Money |
| `state.currencies.eridium` | u64 | Premium currency |
| `state.experience[0].level` | u64 | Character level |
| `state.experience[0].points` | u64 | Character XP |
| `state.experience[1].level` | u64 | Specialization level |
| `state.experience[1].points` | u64 | Specialization XP |
| `state.inventory.items.backpack` | object | Inventory items |

### Experience Types

The `experience` array contains entries with different types:
- Index 0: `Character` - Main character level/XP
- Index 1: `Specialization` - Spec tree level/XP

### Inventory Structure

There are multiple inventory sections in the save file.

#### Equipped Inventory

Currently equipped items are stored in `state.inventory.equipped_inventory.equipped`:

```yaml
equipped_inventory:
  equipped:
    slot_0:
    - serial: '@Ugr...'
      flags: 1
      state_flags: 3
    slot_1:
    - serial: '@Ugx...'
      flags: 1
      state_flags: 515
    # ... slots 2-8
```

#### Equipment Slot Mapping

| Save Slot | UI Position | Item Type |
|-----------|-------------|-----------|
| slot_0 | Weapon 1 (Left) | Weapon |
| slot_1 | Weapon 2 (Top) | Weapon |
| slot_2 | Weapon 3 (Right) | Weapon |
| slot_3 | Weapon 4 (Bottom) | Weapon |
| slot_4 | ? | Equipment |
| slot_5 | ? | Equipment |
| slot_6 | ? | Equipment |
| slot_7 | ? | Equipment |
| slot_8 | ? | Enhancement |

**Status**: All 4 weapon slots confirmed via swap testing.

#### Backpack Inventory

```yaml
inventory:
  items:
    backpack:
      slot_0:
        serial: "@Ugr$ZCm/&tH!t{KgK/Shxu>k"
        flags: 0
      slot_1:
        serial: "@Uge8jxm/)@{!gQaYMipv(G&-b*Z~_"
        flags: 0
```

Each slot contains:
- `serial` - Base85-encoded item data (see structures.md)
- `flags` - Item flags (meaning TBD)
- `state_flags` - State flags (seen values: 1, 3, 515, 513)

## Processing Pipeline

### Decryption (Reading)

```
.sav file
    ↓ Read raw bytes
    ↓ AES-256-ECB decrypt (Steam ID key)
    ↓ Remove PKCS7 padding
    ↓ zlib decompress
    ↓ Parse YAML
SaveFile struct
```

### Encryption (Writing)

```
SaveFile struct
    ↓ Serialize to YAML
    ↓ zlib compress
    ↓ Add PKCS7 padding
    ↓ AES-256-ECB encrypt (Steam ID key)
    ↓ Write bytes
.sav file
```

## Backup System

The bl4 tool maintains backups with metadata:

### Files Created
- `<name>.sav.bak` - Backup of original save
- `<name>.sav.bak.json` - Metadata tracking hashes

### Metadata Format
```json
{
  "original_hash": "<sha256_hex>",
  "last_edit_hash": "<sha256_hex>"
}
```

### Backup Logic
1. First edit: Create backup + metadata
2. Subsequent edits: Update `last_edit_hash` only
3. If file hash doesn't match `original_hash` or `last_edit_hash`: User replaced file, create new backup

## Validation

### Checksums
The save file format does not appear to include internal checksums. Validation is handled by:
- Successful AES decryption (padding validation)
- Successful zlib decompression
- Valid YAML parsing

### Known Constraints
- Steam ID must match the one used to create the save
- Using wrong Steam ID results in garbage after decryption

## Version History

| Game Version | Format Changes |
|--------------|----------------|
| Launch | Initial format documented here |

## Tools

### bl4 CLI Commands

```bash
# Decrypt to YAML
bl4 decrypt 1.sav save.yaml

# Encrypt from YAML
bl4 encrypt save.yaml 1.sav

# Interactive edit
bl4 edit 1.sav

# Query values
bl4 get 1.sav "state.currencies.cash"

# Modify values
bl4 set 1.sav "state.currencies.cash" 999999
```

---

**Last Updated**: Initial creation
**Format Version**: BL4 Launch
