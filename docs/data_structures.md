# Borderlands 4 Data Structures

This document captures reverse engineering findings for Borderlands 4, focusing on item serial encoding and game data structures.

## Table of Contents

1. [Item Serial Format](#item-serial-format)
2. [Reference Tables](#reference-tables)
3. [Game Data Structures](#game-data-structures)
4. [Memory Analysis Details](#memory-analysis-details)
5. [Next Steps](#next-steps)

---

## Item Serial Format

Item serials are encoded strings that fully describe an item's properties. They appear in save files and can be shared between players.

### Serial Structure

```
@Ug<type><base85_data>
```

- **Prefix**: `@Ug` (constant)
- **Type**: Single character indicating item category
- **Data**: Custom Base85 encoded bitstream

### Decoding Pipeline

1. **Strip prefix**: Remove `@U` from the serial string
2. **Base85 decode**: Use custom alphabet, big-endian byte order
3. **Bit mirror**: Reverse bits in each byte (e.g., `0b10000111` → `0b11100001`)
4. **Parse bitstream**: Extract variable-length tokens

### Base85 Alphabet

```
0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>?@^_`{/}~
```

### Bitstream Structure

```
[7-bit magic: 0010000][tokens...][00 terminator][zero padding]
```

The bitstream consists of variable-length tokens with no byte alignment.

### Token Types

| Prefix | Bits | Name | Description |
|--------|------|------|-------------|
| `00` | 2 | Separator | Hard separator, renders as `\|` |
| `01` | 2 | SoftSeparator | Soft separator, renders as `,` |
| `100` | 3 | VarInt | Nibble-based variable integer |
| `101` | 3 | Part | Complex part structure with index and optional values |
| `110` | 3 | VarBit | Bit-length-prefixed integer |
| `111` | 3 | String | Length-prefixed 7-bit ASCII string |

### VarInt Encoding

Reads 4-bit values with continuation bits:

```
[4-bit value][1-bit cont][4-bit value][1-bit cont]...
```

- Continuation bit `1` = more nibbles follow
- Continuation bit `0` = stop
- Maximum 4 nibbles (16 bits total)
- Values assembled LSB-first (shift left by 4 per nibble)

### VarBit Encoding

```
[5-bit length][N-bit value]
```

- Length specifies how many data bits follow
- Length 0 means value is 0
- Value bits read MSB-first from the bitstream

### Part Structure

```
[VarInt index][1-bit type flag]
```

**Type flag = 1:**
```
[VarInt value][000 terminator]
```

**Type flag = 0:**
```
[2-bit subtype]
  Subtype 10: No additional data (empty part)
  Subtype 01: Value list [values...][00 terminator]
```

### String Encoding

```
[VarInt length][7-bit ASCII chars...]
```

Length is the number of characters. Each character is 7-bit ASCII.

### Formatted Output Notation

The `bl4 decode` command outputs tokens in human-readable format:

```
180928 | 50 | {0:1} 1660 | | {8} {14} {252:4} ,
```

| Notation | Meaning |
|----------|---------|
| `12345` | Integer value (VarInt or VarBit) |
| `\|` | Hard separator |
| `,` | Soft separator |
| `{index}` | Part with no value |
| `{index:value}` | Part with single value |
| `{index:[v1 v2]}` | Part with multiple values |
| `"text"` | String token |

### Item Type Characters

| Char | Item Category | First Token Value |
|------|---------------|-------------------|
| `b` | Pistol | VarInt(2) |
| `d` | SMG | VarInt(9) |
| `x` | Shotgun | VarInt(134) |
| `y` | Sniper Rifle | (unknown) |
| `e` | Equipment/Shield | VarBit(107200) |
| `u` | Utility | VarInt(128) |
| `r` | Weapon (generic) | VarBit(180928) |

### Common Serial Structure

Most serials follow this general pattern:

```
[item_subtype], [values...] | [level/info] | [data] || [parts...]
```

The double separator `||` typically precedes the parts list.

**Example breakdown** (type `r` weapon):
```
180928 | 50 | {0:1} 1660 | | {8} {14} {252:4}
  │      │     │     │       └── Parts list
  │      │     │     └── Unknown ID
  │      │     └── Part with value
  │      └── Level (50)
  └── Item subtype ID
```

### CLI Usage

```bash
# Basic decode (shows tokens in formatted notation)
bl4 decode '@Ugr$ZCm/&tH!t{KgK/Shxu>k'

# Verbose output (shows all tokens, raw bytes, extracted fields)
bl4 decode --verbose '@Ugr$ZCm/&tH!t{KgK/Shxu>k'

# Debug mode (shows bit-by-bit parsing to stderr)
bl4 decode --debug '@Ugr$ZCm/&tH!t{KgK/Shxu>k'
```

---

## Reference Tables

### Manufacturer Codes

| Code | Manufacturer | Specialty |
|------|--------------|-----------|
| `BOR` | Unknown | SMGs |
| `DAD` | Daedalus | SMG, Shotgun, AR |
| `DPL` | Dahl | Burst fire |
| `JAK` | Jakobs | High damage, semi-auto |
| `MAL` | Maliwan | Elemental weapons |
| `ORD` | Unknown | Sniper, Pistol, AR |
| `TED` | Tediore | Throwable reloads |
| `TOR` | Torgue | Explosive weapons |
| `VLA` | Vladof | High fire rate |

### Weapon Type Codes

| Code | Type |
|------|------|
| `AR` | Assault Rifle |
| `HW` | Heavy Weapon |
| `PS` | Pistol |
| `SG` | Shotgun |
| `SM` | SMG |
| `SR` | Sniper Rifle |

### Rarity Tiers

| Code | Rarity |
|------|--------|
| `comp_01` | Common |
| `comp_02` | Uncommon |
| `comp_03` | Rare |
| `comp_04` | Epic |
| `comp_05` | Legendary |

### Known Legendary Weapons

| Internal Name | Display Name | Type | Manufacturer |
|---------------|--------------|------|--------------|
| `DAD_AR.comp_05_legendary_OM` | OM | AR | Daedalus |
| `DAD_SG.comp_05_legendary_HeartGUn` | Heart Gun | Shotgun | Daedalus |
| `JAK_AR.comp_05_legendary_rowan` | Rowan's Call | AR | Jakobs |
| `JAK_PS.comp_05_legendary_kingsgambit` | King's Gambit | Pistol | Jakobs |
| `JAK_PS.comp_05_legendary_phantom_flame` | Phantom Flame | Pistol | Jakobs |
| `JAK_SR.comp_05_legendary_ballista` | Ballista | Sniper | Jakobs |
| `MAL_HW.comp_05_legendary_GammaVoid` | Gamma Void | Heavy | Maliwan |
| `MAL_SM.comp_05_legendary_OhmIGot` | Ohm I Got | SMG | Maliwan |
| `TED_AR.comp_05_legendary_Chuck` | Chuck | AR | Tediore |
| `TED_PS.comp_05_legendary_Sideshow` | Sideshow | Pistol | Tediore |
| `TOR_HW.comp_05_legendary_ravenfire` | Ravenfire | Heavy | Torgue |
| `TOR_SG.comp_05_legendary_Linebacker` | Linebacker | Shotgun | Torgue |
| `VLA_AR.comp_05_legendary_WomboCombo` | Wombo Combo | AR | Vladof |
| `VLA_HW.comp_05_legendary_AtlingGun` | Atling Gun | Heavy | Vladof |
| `VLA_SM.comp_05_legendary_KaoSon` | Kaoson | SMG | Vladof |

---

## Game Data Structures

### Weapon Attribute System

BL4 uses an attribute system for weapon stats. Stats are calculated dynamically rather than stored directly.

**Attribute naming pattern:** `Att_<Category>_<Name>`

| Prefix | Category |
|--------|----------|
| `Att_Weapon_*` | Weapon-specific |
| `Att_Calc_*` | Calculated/derived |
| `Att_Grav_*` | Gravitar class |
| `Att_PLD_*` | Paladin class |

**Key stat properties:**

| Property | Description |
|----------|-------------|
| `BaseDamage` | Base weapon damage |
| `DamagePerShot` | Per-projectile damage |
| `ProjectilesPerShot` | Pellet count (x4, x6, etc.) |
| `Accuracy` | Weapon accuracy |
| `AccuracyImpulse` | Accuracy impulse modifier |
| `FireRate` | Firing rate |
| `ReloadTime` | Reload time |

### Weapon Part System

Weapons are composed of multiple parts, each affecting stats.

**Part categories:**
- Barrel
- Grip
- Stock
- Scope
- UnderBarrel
- Accessory

**Part data classes:**

| Class | Purpose |
|-------|---------|
| `GestaltPartDataSelector` | Part selection logic |
| `GestaltRandomPartData` | Random part generation |
| `GestaltOptionalPartData` | Optional part handling |
| `PartData` | Base part data |
| `PartList` | Available parts list |

**Stat calculation:**
```
Final Stat = Base Value × Part Modifier₁ × Part Modifier₂ × ...
```

Part modifiers are typically floats in the 0.5-1.0 range.

### Loot System

**ItemPool classes:**

| Class | Description |
|-------|-------------|
| `ItemPoolDef` | Defines a loot pool |
| `ItemPoolEntry` | Single pool entry |
| `ItemPoolListDef` | List of pools |
| `ItemPoolSelectorDef` | Selection logic |

**Weight properties:**

| Property | Description |
|----------|-------------|
| `BaseWeight` | Base drop weight |
| `RarityWeight` | Weight by rarity |
| `GrowthExponent` | Level scaling |
| `GameStageVariance` | Stage variance |

**Luck system:**
- `LuckCategories` - Luck modifier categories
- `EnemyBasedLuckCategories` - Enemy-specific
- `PlayerBasedLuckCategories` - Player-specific

---

## Memory Analysis Details

This section documents findings from memory dump analysis for future reference.

### Environment

- **Process**: `Borderlands4.exe` under Wine/Proton
- **Dump method**: `gcore`
- **Dump files**:
  - `share/dumps/vex_level50_uvh5_shotguns.dump.107180` (8.3 GB)
  - `share/dumps/vex_level50_uvh5_bank.dump.107180` (26 GB)

### Serial Storage in Memory

Serials stored as length-prefixed strings:

```
[2-byte LE length][serial string][2-byte LE length][serial string]...
```

Length includes `@Ug` prefix and null terminator.

### Item Entry Structure (Runtime)

Observed at offset `0x014cd0c8`:

```
Offset | Size | Content
-------|------|--------
-0x10  | 8    | Header/flags
-0x08  | 4    | Float (unknown)
0x00   | var  | Serial string (null-terminated)
+len+2 | 8    | Length/type info
+0x0A  | 4    | ID or hash
+0x0E  | 2    | Version/flags
+0x10  | 4    | Float (possibly level)
```

### Stat Modifier Floats

Found near item entries (offset `0x014cd018`):

| Float | Possible Use |
|-------|--------------|
| 1.0000 | Base multiplier |
| 0.8078 | Stat modifier |
| 0.8510 | Stat modifier |
| 0.8941 | Stat modifier |
| 0.5647 | Stat modifier |

### Serial Byte Correlation

Comparing two Linebacker shotguns with different stats:

| Stat | Slot 3 | Slot 4 |
|------|--------|--------|
| Accuracy | 71% | 74% |
| Reload | 1.8s | 1.6s |
| Byte 9 | 0x71 | 0xb1 |

First bit divergence at bit 72 correlates with stat differences.

---

## Part Slot Types (from Memory)

Found `EWeaponPartValue` enum defining part slots:

| Slot | Description |
|------|-------------|
| Grip | Weapon grip |
| Foregrip | Front grip |
| Reload | Reload mechanism |
| Barrel | Main barrel |
| Scope | Optics/scope |
| Melee | Melee attachment |
| Mode | Fire mode |
| ModeSwitch | Mode switch mechanism |
| Underbarrel | Under-barrel attachment |
| Custom0-7 | Additional custom slots |

### Memory Dump Findings

The memory dump contains:
- Class definitions and enum values
- Weapon asset paths (e.g., `TOR_SG_Scope_01_L2_B`, `BOR_HW_Barrel_02`)
- Runtime part structures

**Not found**: Explicit part index → asset name mapping tables. These are likely stored in PAK files.

## Next Steps

### High Priority

1. **Extract PAK files** - Use FModel or UModel to extract weapon/part definitions
2. **Build part index table** - Map Part tokens `{index}` to actual game parts
3. **Empirical testing** - Compare serials of items with known part differences

### Medium Priority

4. **Trace attribute resolution** - Understand how `BaseDamage` etc. resolve to final values
5. **Decode more item types** - Verify patterns hold for all weapon/equipment types
6. **Serial encoding** - Implement the reverse (encoding) to create/modify items

### Future Work

7. **PAK file analysis** - Extract part definitions from game assets
8. **Static binary analysis** - Use radare2 to find serialization code
9. **WASM bindings** - Expose serial decoding to browser-based editor

---

*Last updated during memory dump analysis session*
