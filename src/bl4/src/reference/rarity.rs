//! Rarity tier definitions

/// Rarity tier information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RarityTier {
    pub tier: u8,
    pub code: &'static str,
    pub name: &'static str,
    pub color: &'static str,
}

/// All rarity tiers in order
pub const RARITY_TIERS: &[RarityTier] = &[
    RarityTier {
        tier: 1,
        code: "comp_01",
        name: "Common",
        color: "#FFFFFF",
    },
    RarityTier {
        tier: 2,
        code: "comp_02",
        name: "Uncommon",
        color: "#00FF00",
    },
    RarityTier {
        tier: 3,
        code: "comp_03",
        name: "Rare",
        color: "#0080FF",
    },
    RarityTier {
        tier: 4,
        code: "comp_04",
        name: "Epic",
        color: "#A020F0",
    },
    RarityTier {
        tier: 5,
        code: "comp_05",
        name: "Legendary",
        color: "#FFA500",
    },
];

/// Get rarity tier by tier number
pub fn rarity_by_tier(tier: u8) -> Option<&'static RarityTier> {
    RARITY_TIERS.iter().find(|r| r.tier == tier)
}

/// Get rarity tier by code
pub fn rarity_by_code(code: &str) -> Option<&'static RarityTier> {
    RARITY_TIERS.iter().find(|r| r.code == code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rarity_lookup() {
        assert_eq!(rarity_by_tier(1).map(|r| r.name), Some("Common"));
        assert_eq!(rarity_by_tier(5).map(|r| r.name), Some("Legendary"));
        assert_eq!(rarity_by_code("comp_03").map(|r| r.name), Some("Rare"));
    }
}
