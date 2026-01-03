//! Serialization types for Zen asset parsing

use serde::Serialize;

/// Parsed asset information in JSON-friendly format
#[derive(Debug, Serialize)]
pub struct ZenAssetInfo {
    pub path: String,
    pub package_name: String,
    pub package_flags: u32,
    pub is_unversioned: bool,
    pub name_count: usize,
    pub import_count: usize,
    pub export_count: usize,
    pub names: Vec<String>,
    pub imports: Vec<ZenImportInfo>,
    pub exports: Vec<ZenExportInfo>,
}

/// Import reference information
#[derive(Debug, Serialize)]
pub struct ZenImportInfo {
    pub index: usize,
    pub type_name: String,
}

/// Export object information with optional parsed properties
#[derive(Debug, Serialize)]
pub struct ZenExportInfo {
    pub index: usize,
    pub object_name: String,
    pub class_index: String,
    pub super_index: String,
    pub template_index: String,
    pub outer_index: String,
    pub public_export_hash: u64,
    pub cooked_serial_offset: u64,
    pub cooked_serial_size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<ParsedProperty>>,
}

/// Parsed property with type-specific value fields
#[derive(Debug, Serialize, Clone)]
pub struct ParsedProperty {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int_value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_values: Option<Vec<ParsedPropertyValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub struct_values: Option<Vec<ParsedProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_values: Option<Vec<(ParsedPropertyValue, ParsedPropertyValue)>>,
}

impl ParsedProperty {
    /// Create an empty property with just a name
    pub fn new() -> Self {
        Self {
            name: String::new(),
            value_type: None,
            float_value: None,
            int_value: None,
            string_value: None,
            object_path: None,
            array_values: None,
            struct_values: None,
            enum_value: None,
            map_values: None,
        }
    }

    /// Create a property with a specific type
    pub fn with_type(type_name: &str) -> Self {
        Self {
            name: String::new(),
            value_type: Some(type_name.to_string()),
            float_value: None,
            int_value: None,
            string_value: None,
            object_path: None,
            array_values: None,
            struct_values: None,
            enum_value: None,
            map_values: None,
        }
    }
}

impl Default for ParsedProperty {
    fn default() -> Self {
        Self::new()
    }
}

/// Simplified property value for arrays and maps (no name needed)
#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum ParsedPropertyValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Object(String),
    Array(Vec<ParsedPropertyValue>),
    Struct(Vec<ParsedProperty>),
}

/// Extract property info from asset name table
/// Property names in DataTables follow the pattern: PropertyName_Index_GUID
#[derive(Debug, Clone)]
pub struct PropertyTypeInfo {
    pub name: String,
    pub type_name: String,
    pub size: usize,
    pub schema_index: u32,
}
