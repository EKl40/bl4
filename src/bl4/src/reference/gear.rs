//! Gear type definitions

/// Gear type information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GearType {
    pub code: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}

/// All gear types
pub const GEAR_TYPES: &[GearType] = &[
    GearType {
        code: "shield",
        name: "Shield",
        description: "Defensive equipment",
    },
    GearType {
        code: "classmod",
        name: "Class Mod",
        description: "Character class modifications",
    },
    GearType {
        code: "enhancement",
        name: "Enhancement",
        description: "Permanent character upgrades",
    },
    GearType {
        code: "gadget",
        name: "Gadget",
        description: "Deployable equipment",
    },
    GearType {
        code: "repair_kit",
        name: "Repair Kit",
        description: "Healing items",
    },
    GearType {
        code: "grenade",
        name: "Grenade",
        description: "Throwable explosive devices",
    },
];

/// Get gear type by code
pub fn gear_type_by_code(code: &str) -> Option<&'static GearType> {
    GEAR_TYPES.iter().find(|g| g.code == code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gear_type_by_code() {
        assert_eq!(gear_type_by_code("shield").map(|g| g.name), Some("Shield"));
        assert_eq!(
            gear_type_by_code("classmod").map(|g| g.name),
            Some("Class Mod")
        );
        assert_eq!(
            gear_type_by_code("grenade").map(|g| g.name),
            Some("Grenade")
        );
        assert_eq!(
            gear_type_by_code("enhancement").map(|g| g.name),
            Some("Enhancement")
        );
        assert_eq!(gear_type_by_code("gadget").map(|g| g.name), Some("Gadget"));
        assert_eq!(
            gear_type_by_code("repair_kit").map(|g| g.name),
            Some("Repair Kit")
        );
        assert!(gear_type_by_code("unknown").is_none());
    }
}
