//! Weapon type definitions

/// Weapon type information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaponType {
    pub code: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}

/// All weapon types
pub const WEAPON_TYPES: &[WeaponType] = &[
    WeaponType {
        code: "AR",
        name: "Assault Rifle",
        description: "Full-auto/burst fire rifles",
    },
    WeaponType {
        code: "HW",
        name: "Heavy Weapon",
        description: "Launchers and miniguns",
    },
    WeaponType {
        code: "PS",
        name: "Pistol",
        description: "Semi-auto and full-auto handguns",
    },
    WeaponType {
        code: "SG",
        name: "Shotgun",
        description: "High-damage spread weapons",
    },
    WeaponType {
        code: "SM",
        name: "SMG",
        description: "Submachine guns",
    },
    WeaponType {
        code: "SR",
        name: "Sniper Rifle",
        description: "Long-range precision weapons",
    },
];

/// Get weapon type by code
pub fn weapon_type_by_code(code: &str) -> Option<&'static WeaponType> {
    WEAPON_TYPES.iter().find(|w| w.code == code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weapon_type_lookup() {
        assert_eq!(
            weapon_type_by_code("AR").map(|w| w.name),
            Some("Assault Rifle")
        );
        assert_eq!(
            weapon_type_by_code("SR").map(|w| w.name),
            Some("Sniper Rifle")
        );
    }
}
