//! State flags bitmask helper for inventory items.
//!
//! Items in Borderlands 4 saves have a `state_flags` field that encodes
//! various properties using a bitmask. This struct provides a type-safe
//! way to work with these flags without knowing the bit positions.
//!
//! # Example
//! ```
//! use bl4::StateFlags;
//!
//! // Create flags for a backpack item marked as favorite
//! let flags = StateFlags::backpack().with_favorite();
//!
//! // Create flags for an equipped item
//! let equipped = StateFlags::equipped();
//!
//! // Query flags
//! assert!(flags.is_favorite());
//! assert!(flags.is_in_backpack());
//! ```

/// State flags bitmask helper for inventory items.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StateFlags(pub u32);

impl StateFlags {
    // Bit values matching Borderlands 4's state_flags field (verified in-game)
    const VALID: u32 = 1; // bit 0 - item exists/valid
    const FAVORITE: u32 = 2; // bit 1 - favorite
    const JUNK: u32 = 4; // bit 2 - junk marker
    const LABEL1: u32 = 16; // bit 4 - label 1
    const LABEL2: u32 = 32; // bit 5 - label 2
    const LABEL3: u32 = 64; // bit 6 - label 3
    const LABEL4: u32 = 128; // bit 7 - label 4
    const IN_BACKPACK: u32 = 512; // bit 9 - in backpack (not equipped)

    // All label bits are mutually exclusive (only one can be set at a time)
    const ALL_LABELS: u32 =
        Self::FAVORITE | Self::JUNK | Self::LABEL1 | Self::LABEL2 | Self::LABEL3 | Self::LABEL4;

    /// Create flags for a backpack item (valid + in_backpack).
    pub fn backpack() -> Self {
        Self(Self::VALID | Self::IN_BACKPACK)
    }

    /// Create flags for an equipped item (valid only, no backpack bit).
    pub fn equipped() -> Self {
        Self(Self::VALID)
    }

    /// Create flags for a bank item (valid only).
    pub fn bank() -> Self {
        Self(Self::VALID)
    }

    /// Create flags from a raw u32 value.
    pub fn from_raw(bits: u32) -> Self {
        Self(bits)
    }

    /// Get the raw u32 value.
    pub fn to_raw(self) -> u32 {
        self.0
    }

    // Builder methods (chainable)
    // Note: Labels are mutually exclusive - setting one clears others

    /// Set the favorite label (clears other labels).
    pub fn with_favorite(mut self) -> Self {
        self.0 = (self.0 & !Self::ALL_LABELS) | Self::FAVORITE;
        self
    }

    /// Set the junk label (clears other labels).
    pub fn with_junk(mut self) -> Self {
        self.0 = (self.0 & !Self::ALL_LABELS) | Self::JUNK;
        self
    }

    /// Set label 1 (clears other labels).
    pub fn with_label1(mut self) -> Self {
        self.0 = (self.0 & !Self::ALL_LABELS) | Self::LABEL1;
        self
    }

    /// Set label 2 (clears other labels).
    pub fn with_label2(mut self) -> Self {
        self.0 = (self.0 & !Self::ALL_LABELS) | Self::LABEL2;
        self
    }

    /// Set label 3 (clears other labels).
    pub fn with_label3(mut self) -> Self {
        self.0 = (self.0 & !Self::ALL_LABELS) | Self::LABEL3;
        self
    }

    /// Set label 4 (clears other labels).
    pub fn with_label4(mut self) -> Self {
        self.0 = (self.0 & !Self::ALL_LABELS) | Self::LABEL4;
        self
    }

    /// Clear all labels (favorite, junk, 1-4).
    pub fn with_no_label(mut self) -> Self {
        self.0 &= !Self::ALL_LABELS;
        self
    }

    // Query methods

    /// Check if the favorite flag is set.
    pub fn is_favorite(&self) -> bool {
        self.0 & Self::FAVORITE != 0
    }

    /// Check if the junk flag is set.
    pub fn is_junk(&self) -> bool {
        self.0 & Self::JUNK != 0
    }

    /// Check if label 1 is set.
    pub fn has_label1(&self) -> bool {
        self.0 & Self::LABEL1 != 0
    }

    /// Check if label 2 is set.
    pub fn has_label2(&self) -> bool {
        self.0 & Self::LABEL2 != 0
    }

    /// Check if label 3 is set.
    pub fn has_label3(&self) -> bool {
        self.0 & Self::LABEL3 != 0
    }

    /// Check if label 4 is set.
    pub fn has_label4(&self) -> bool {
        self.0 & Self::LABEL4 != 0
    }

    /// Check if the item is in backpack (not equipped).
    pub fn is_in_backpack(&self) -> bool {
        self.0 & Self::IN_BACKPACK != 0
    }

    /// Check if the item is equipped (not in backpack only).
    pub fn is_equipped(&self) -> bool {
        !self.is_in_backpack()
    }

    // Mutation methods
    // Note: Labels are mutually exclusive - setting one clears others

    /// Set favorite label (clears other labels) or clear it.
    pub fn set_favorite(&mut self, value: bool) {
        if value {
            self.0 = (self.0 & !Self::ALL_LABELS) | Self::FAVORITE;
        } else {
            self.0 &= !Self::FAVORITE;
        }
    }

    /// Set junk label (clears other labels) or clear it.
    pub fn set_junk(&mut self, value: bool) {
        if value {
            self.0 = (self.0 & !Self::ALL_LABELS) | Self::JUNK;
        } else {
            self.0 &= !Self::JUNK;
        }
    }

    /// Set label 1 (clears other labels) or clear it.
    pub fn set_label1(&mut self, value: bool) {
        if value {
            self.0 = (self.0 & !Self::ALL_LABELS) | Self::LABEL1;
        } else {
            self.0 &= !Self::LABEL1;
        }
    }

    /// Set label 2 (clears other labels) or clear it.
    pub fn set_label2(&mut self, value: bool) {
        if value {
            self.0 = (self.0 & !Self::ALL_LABELS) | Self::LABEL2;
        } else {
            self.0 &= !Self::LABEL2;
        }
    }

    /// Set label 3 (clears other labels) or clear it.
    pub fn set_label3(&mut self, value: bool) {
        if value {
            self.0 = (self.0 & !Self::ALL_LABELS) | Self::LABEL3;
        } else {
            self.0 &= !Self::LABEL3;
        }
    }

    /// Set label 4 (clears other labels) or clear it.
    pub fn set_label4(&mut self, value: bool) {
        if value {
            self.0 = (self.0 & !Self::ALL_LABELS) | Self::LABEL4;
        } else {
            self.0 &= !Self::LABEL4;
        }
    }

    /// Clear all labels.
    pub fn clear_labels(&mut self) {
        self.0 &= !Self::ALL_LABELS;
    }

    /// Convert to equipped flags (clear backpack bit).
    pub fn to_equipped(mut self) -> Self {
        self.0 &= !Self::IN_BACKPACK;
        self
    }

    /// Convert to backpack flags (set backpack bit).
    pub fn to_backpack(mut self) -> Self {
        self.0 |= Self::IN_BACKPACK;
        self
    }
}

impl From<u32> for StateFlags {
    fn from(v: u32) -> Self {
        Self(v)
    }
}

impl From<StateFlags> for u32 {
    fn from(f: StateFlags) -> Self {
        f.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_flags_backpack() {
        let flags = StateFlags::backpack();
        assert_eq!(flags.0, 513); // 1 (valid) + 512 (in_backpack)
        assert!(flags.is_in_backpack());
        assert!(!flags.is_equipped());
    }

    #[test]
    fn test_state_flags_equipped() {
        let flags = StateFlags::equipped();
        assert_eq!(flags.0, 1); // just valid
        assert!(!flags.is_in_backpack());
        assert!(flags.is_equipped());
    }

    #[test]
    fn test_state_flags_bank() {
        let flags = StateFlags::bank();
        assert_eq!(flags.0, 1); // just valid
    }

    #[test]
    fn test_state_flags_with_favorite() {
        let flags = StateFlags::backpack().with_favorite();
        assert_eq!(flags.0, 515); // 513 + 2
        assert!(flags.is_favorite());
        assert!(flags.is_in_backpack());
    }

    #[test]
    fn test_state_flags_with_junk() {
        let flags = StateFlags::backpack().with_junk();
        assert_eq!(flags.0, 517); // 513 + 4
        assert!(flags.is_junk());
    }

    #[test]
    fn test_state_flags_labels() {
        // Labels are mutually exclusive - only the last one set should be active
        let flags = StateFlags::backpack()
            .with_label2()
            .with_label3()
            .with_label4(); // Only label4 remains
        assert!(!flags.has_label2());
        assert!(!flags.has_label3());
        assert!(flags.has_label4());
        assert_eq!(flags.0, 513 + 128); // backpack + label4

        // Verify each label clears the others
        let fav = StateFlags::backpack().with_favorite();
        assert_eq!(fav.0, 515); // 513 + 2

        let junk = fav.with_junk(); // Changes from favorite to junk
        assert!(!junk.is_favorite());
        assert!(junk.is_junk());
        assert_eq!(junk.0, 517); // 513 + 4
    }

    #[test]
    fn test_state_flags_mutation() {
        let mut flags = StateFlags::backpack();
        assert!(!flags.is_favorite());

        flags.set_favorite(true);
        assert!(flags.is_favorite());
        assert_eq!(flags.0, 515); // 513 + 2 (favorite)

        flags.set_favorite(false);
        assert!(!flags.is_favorite());
        assert_eq!(flags.0, 513);
    }

    #[test]
    fn test_state_flags_to_equipped() {
        let backpack = StateFlags::backpack().with_favorite();
        let equipped = backpack.to_equipped();
        assert!(!equipped.is_in_backpack());
        assert!(equipped.is_favorite()); // preserves other flags
        assert_eq!(equipped.0, 3); // 1 + 2 (valid + favorite)
    }

    #[test]
    fn test_state_flags_to_backpack() {
        let equipped = StateFlags::equipped().with_junk();
        let backpack = equipped.to_backpack();
        assert!(backpack.is_in_backpack());
        assert!(backpack.is_junk()); // preserves other flags
        assert_eq!(backpack.0, 517); // 513 + 4 (backpack + junk)
    }

    #[test]
    fn test_state_flags_from_raw() {
        let flags = StateFlags::from_raw(515); // 513 + 2 = backpack + favorite
        assert!(flags.is_in_backpack());
        assert!(flags.is_favorite());
    }

    #[test]
    fn test_state_flags_conversions() {
        let flags = StateFlags::backpack();
        let raw: u32 = flags.into();
        assert_eq!(raw, 513);

        let restored: StateFlags = 515.into(); // 513 + 2 = backpack + favorite
        assert!(restored.is_favorite());
    }

    #[test]
    fn test_state_flags_set_junk_mutation() {
        let mut flags = StateFlags::backpack();
        flags.set_junk(true);
        assert!(flags.is_junk());
        assert!(!flags.is_favorite()); // Labels are mutually exclusive

        flags.set_junk(false);
        assert!(!flags.is_junk());
    }

    #[test]
    fn test_state_flags_set_label1() {
        let mut flags = StateFlags::backpack();
        flags.set_label1(true);
        assert!(flags.has_label1());
        assert!(!flags.is_favorite());

        flags.set_label1(false);
        assert!(!flags.has_label1());
    }

    #[test]
    fn test_state_flags_set_label2() {
        let mut flags = StateFlags::backpack();
        flags.set_label2(true);
        assert!(flags.has_label2());

        flags.set_label2(false);
        assert!(!flags.has_label2());
    }

    #[test]
    fn test_state_flags_set_label3() {
        let mut flags = StateFlags::backpack();
        flags.set_label3(true);
        assert!(flags.has_label3());

        flags.set_label3(false);
        assert!(!flags.has_label3());
    }

    #[test]
    fn test_state_flags_set_label4() {
        let mut flags = StateFlags::backpack();
        flags.set_label4(true);
        assert!(flags.has_label4());

        flags.set_label4(false);
        assert!(!flags.has_label4());
    }

    #[test]
    fn test_state_flags_clear_labels() {
        let mut flags = StateFlags::backpack().with_favorite();
        assert!(flags.is_favorite());

        flags.clear_labels();
        assert!(!flags.is_favorite());
        assert!(!flags.is_junk());
        assert!(!flags.has_label1());
        assert!(flags.is_in_backpack()); // Non-label flags preserved
    }

    #[test]
    fn test_state_flags_with_label1() {
        let flags = StateFlags::backpack().with_label1();
        assert!(flags.has_label1());
        assert!(!flags.is_favorite());
        assert!(!flags.is_junk());
    }

    #[test]
    fn test_state_flags_with_no_label() {
        let flags = StateFlags::backpack().with_favorite().with_no_label();
        assert!(!flags.is_favorite());
        assert!(!flags.is_junk());
        assert!(!flags.has_label1());
        assert!(flags.is_in_backpack());
    }

    #[test]
    fn test_state_flags_to_raw() {
        let flags = StateFlags::backpack();
        assert_eq!(flags.to_raw(), 513);
    }
}
