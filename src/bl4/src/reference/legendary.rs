//! Known legendary item definitions

/// Known legendary item
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegendaryItem {
    pub internal: &'static str,
    pub name: &'static str,
    pub weapon_type: &'static str,
    pub manufacturer: &'static str,
}

/// Known legendary items
pub const KNOWN_LEGENDARIES: &[LegendaryItem] = &[
    // Daedalus
    LegendaryItem {
        internal: "DAD_AR.comp_05_legendary_OM",
        name: "OM",
        weapon_type: "AR",
        manufacturer: "DAD",
    },
    LegendaryItem {
        internal: "DAD_AR_Lumberjack",
        name: "Lumberjack",
        weapon_type: "AR",
        manufacturer: "DAD",
    },
    LegendaryItem {
        internal: "DAD_SG.comp_05_legendary_HeartGUn",
        name: "Heart Gun",
        weapon_type: "SG",
        manufacturer: "DAD",
    },
    LegendaryItem {
        internal: "DAD_PS.Zipper",
        name: "Zipper",
        weapon_type: "PS",
        manufacturer: "DAD",
    },
    LegendaryItem {
        internal: "DAD_PS.Rangefinder",
        name: "Rangefinder",
        weapon_type: "PS",
        manufacturer: "DAD",
    },
    LegendaryItem {
        internal: "DAD_SG.Durendal",
        name: "Durendal",
        weapon_type: "SG",
        manufacturer: "DAD",
    },
    // Jakobs
    LegendaryItem {
        internal: "JAK_AR.comp_05_legendary_rowan",
        name: "Rowan's Call",
        weapon_type: "AR",
        manufacturer: "JAK",
    },
    LegendaryItem {
        internal: "JAK_PS.comp_05_legendary_SeventhSense",
        name: "Seventh Sense",
        weapon_type: "PS",
        manufacturer: "JAK",
    },
    LegendaryItem {
        internal: "JAK_PS.comp_05_legendary_kingsgambit",
        name: "King's Gambit",
        weapon_type: "PS",
        manufacturer: "JAK",
    },
    LegendaryItem {
        internal: "JAK_PS.comp_05_legendary_phantom_flame",
        name: "Phantom Flame",
        weapon_type: "PS",
        manufacturer: "JAK",
    },
    LegendaryItem {
        internal: "JAK_SG.comp_05_legendary_RainbowVomit",
        name: "Rainbow Vomit",
        weapon_type: "SG",
        manufacturer: "JAK",
    },
    LegendaryItem {
        internal: "JAK_SR.comp_05_legendary_ballista",
        name: "Ballista",
        weapon_type: "SR",
        manufacturer: "JAK",
    },
    // Maliwan
    LegendaryItem {
        internal: "MAL_HW.comp_05_legendary_GammaVoid",
        name: "Gamma Void",
        weapon_type: "HW",
        manufacturer: "MAL",
    },
    LegendaryItem {
        internal: "MAL_SM.comp_05_legendary_OhmIGot",
        name: "Ohm I Got",
        weapon_type: "SM",
        manufacturer: "MAL",
    },
    // Borg
    LegendaryItem {
        internal: "BOR_SM.comp_05_legendary_p",
        name: "Unknown Borg SMG",
        weapon_type: "SM",
        manufacturer: "BOR",
    },
    // Tediore
    LegendaryItem {
        internal: "TED_AR.comp_05_legendary_Chuck",
        name: "Chuck",
        weapon_type: "AR",
        manufacturer: "TED",
    },
    LegendaryItem {
        internal: "TED_PS.comp_05_legendary_Sideshow",
        name: "Sideshow",
        weapon_type: "PS",
        manufacturer: "TED",
    },
    LegendaryItem {
        internal: "TED_SG.comp_05_legendary_a",
        name: "Unknown Tediore Shotgun",
        weapon_type: "SG",
        manufacturer: "TED",
    },
    // Torgue
    LegendaryItem {
        internal: "TOR_AR.comp_05_legendary_Trogdor",
        name: "Trogdor",
        weapon_type: "AR",
        manufacturer: "TOR",
    },
    LegendaryItem {
        internal: "TOR_HW.comp_05_legendary_ravenfire",
        name: "Ravenfire",
        weapon_type: "HW",
        manufacturer: "TOR",
    },
    LegendaryItem {
        internal: "TOR_SG.comp_05_legendary_Linebacker",
        name: "Linebacker",
        weapon_type: "SG",
        manufacturer: "TOR",
    },
    // Vladof
    LegendaryItem {
        internal: "VLA_AR.comp_05_legendary_WomboCombo",
        name: "Wombo Combo",
        weapon_type: "AR",
        manufacturer: "VLA",
    },
    LegendaryItem {
        internal: "VLA_HW.comp_05_legendary_AtlingGun",
        name: "Atling Gun",
        weapon_type: "HW",
        manufacturer: "VLA",
    },
    LegendaryItem {
        internal: "VLA_SM.comp_05_legendary_KaoSon",
        name: "Kaoson",
        weapon_type: "SM",
        manufacturer: "VLA",
    },
    LegendaryItem {
        internal: "VLA_SR.comp_05_legendary_Vyudazy",
        name: "Vyudazy",
        weapon_type: "SR",
        manufacturer: "VLA",
    },
];

/// Find legendary by internal name
pub fn legendary_by_internal(internal: &str) -> Option<&'static LegendaryItem> {
    KNOWN_LEGENDARIES.iter().find(|l| l.internal == internal)
}

/// Find legendary by display name
pub fn legendary_by_name(name: &str) -> Option<&'static LegendaryItem> {
    KNOWN_LEGENDARIES.iter().find(|l| l.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legendary_lookup() {
        assert!(legendary_by_name("Seventh Sense").is_some());
        assert!(legendary_by_internal("JAK_PS.comp_05_legendary_SeventhSense").is_some());
    }
}
