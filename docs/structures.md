# Borderlands 4 Memory Layout & Data Structures

This document tracks reverse engineering findings about BL4's internal data structures.

## Item Serial Format

### Overview

Items are serialized as Base85-encoded bitstreams with the format:
```
@Ug<type><base85_data>
```

### Type Characters

| Char | Type | Description |
|------|------|-------------|
| `b` | Weapon | Pistol |
| `d` | Weapon | SMG |
| `e` | Equipment | Shields, class mods, artifacts |
| `r` | Weapon | (unknown subtype) |
| `u` | Utility | Consumables, ammo, etc. |
| `x` | Weapon | Shotgun |
| `y` | Weapon | Sniper Rifle |
| `!` | Enhancement | Enhancement slot item |

**Note**: Weapon types are split by category, not a single `r` code. The type character indicates weapon class.

### Base85 Alphabet

Custom 85-character alphabet (NOT standard ASCII85):
```
0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>?@^_`{/}~
```

### Decoding Pipeline

1. **Strip prefix** - Remove `@Ug` and extract type character
2. **Base85 decode** - 5 chars → 4 bytes, big-endian
3. **Bit mirror** - Reverse bit order in each byte (0b10000111 → 0b11100001)
4. **Parse bitstream** - Read tokens MSB-first

### Bitstream Structure

#### Magic Header
First 7 bits must be `0010000` (0x10)

#### Token Types

| Prefix | Type | Format |
|--------|------|--------|
| `00` | Separator | Terminator, ends parsing |
| `01` | SoftSeparator | Section boundary |
| `100` | VarInt | 4-bit nibbles with continuation |
| `101` | Part | Index + value(s) |
| `110` | VarBit | 5-bit length + N bits of data |
| `111` | String | VarInt length + 7-bit ASCII bytes |

#### VarInt Encoding
```
[4-bit value][1-bit cont][4-bit value][1-bit cont]...  (max 4 nibbles)
- cont=1: more nibbles follow
- cont=0: last nibble
Values assembled LSB-first (shift left by 4 per nibble)
```

#### VarBit Encoding
```
[5-bit length][N bits of data]
Length specifies how many data bits follow
```

#### Part Structure
```
[101][varint:index][flag:1]
if flag=0: [varint:single_value]
if flag=1: [varint:count][varint:value]... (count times)
```

## Token Semantics (Work in Progress)

### Known/Suspected Field Order

Based on heuristic analysis, first VarInts appear to be:
1. **Manufacturer ID** - Which company made the item
2. **Rarity tier** - Common, Uncommon, Rare, Epic, Legendary, etc.
3. **Item level** - Required/scaling level

**Status: UNVERIFIED** - Need correlation with known items

### Manufacturer IDs

| ID | Manufacturer | Notes |
|----|--------------|-------|
| 6 | ? | Rainbow Vomit shotgun |
| 25 | ? | Kaoson SMG |
| 28 | Jakobs | Seventh Sense pistol (confirmed - has "crits ricochet" perk) |
| 97 | ? | Asher's Rise sniper |

### Rarity Values

| Value | Rarity | Color |
|-------|--------|-------|
| ? | ? | TBD |

### Weapon Serial Byte Layout (Pistol Example)

Based on analysis of 4 Seventh Sense (Jakobs pistol) variants:

```
Bytes 0-6:  Common header (type, manufacturer)
            21 30 c0 32 0c 4e 08

Byte 7:     Unknown (0x86 or 0x87)
Byte 8:     Element + parts encoding (see below)
Bytes 9+:   Variable part data
```

#### Byte 8 Analysis (Element/Parts)

| Byte 8 | Binary | Element | Licensed Part |
|--------|--------|---------|---------------|
| `0x4b` | `0100 1011` | Kinetic | None |
| `0x74` | `0111 0100` | Corrosive | Hyperion |
| `0x04` | `0000 0100` | Corrosive | Tediore |
| `0x34` | `0011 0100` | Kinetic | None |

**Observation**: Corrosive weapons both have low nibble `0100`. Element may be encoded in bits 2-4.

### SMG Element Analysis

Comparing two SMGs of same base type (type code `x`) with different elements:

| Weapon | Element | Byte 9 | Binary |
|--------|---------|--------|--------|
| Kaoson | Fire | `0x2a` | `0010 1010` |
| Totalitarian | Cryo | `0xaa` | `1010 1010` |

**Key finding**: Bytes 0-8 are identical. Byte 9 differs only in bit 7:
- Fire: bit 7 = 0
- Cryo: bit 7 = 1

This suggests element is encoded starting around bit 72 (byte 9) in the bitstream.

### Licensed Accessories

| Manufacturer | Effect |
|--------------|--------|
| Hyperion | ? |
| Tediore | ? |

### Part Indices

Part structures likely reference:
- Barrel types
- Grip types
- Stock types
- Scope/sight
- Element (possibly byte 8, low nibble)
- Prefix/title parts
- Licensed accessories
- Alternate fire modes (e.g., vial launcher for corrosive)

**Status: Partial - element encoding suspected in byte 8

## Memory Layout (Game Process)

### Finding Items in Memory

Using `scanmem` approach:
1. Search for known item stat (damage, magazine size)
2. Change item (level up, reroll)
3. Filter for changed values
4. Trace memory region to find item structure

### Suspected Item Structure

```c
// Hypothetical - needs verification
struct Item {
    uint32_t item_id;       // Internal ID
    uint8_t  type;          // Weapon/equipment/etc
    uint8_t  rarity;        // Rarity tier
    uint16_t level;         // Item level
    uint32_t manufacturer;  // Manufacturer ID
    // ... parts array
    // ... stats
    // ... serial cache?
};
```

## Serialization Code (Static Analysis)

### Target Functions

To find with radare2/Ghidra:
- Serial encoding function (item → string)
- Serial decoding function (string → item)
- Part lookup tables
- Manufacturer string tables

### Signatures to Search

```
// String patterns
"@Ug"                    // Serial prefix
"0123456789ABCDEF..."    // Base85 alphabet (partial)

// Code patterns
// XOR with bit reversal
// Base85 encode/decode loops
```

## Inventory Structure

### Save File Path
```yaml
state:
  inventory:
    items:
      backpack:
        slot_0:
          serial: "@Ugr..."
          flags: 0
        slot_1:
          serial: "@Uge..."
          flags: 0
```

### Slot Types
- `backpack` - Main inventory
- `equipped` - Currently equipped items (TBD)
- `bank` - Shared storage (TBD)

## Notes & Findings

### Session Log

*Add findings here as we reverse engineer*

---

**Last Updated**: Initial creation
**Contributors**: Claude Code session
