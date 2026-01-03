//! Output formatting for NCS commands

use bl4_ncs::Value;
use std::fmt::Write;

/// Output parsed document as TSV (tab-separated values) to stdout
pub fn output_tsv(doc: &bl4_ncs::Document) {
    print!("{}", format_tsv(doc));
}

/// Format parsed document as TSV string
pub fn format_tsv(doc: &bl4_ncs::Document) -> String {
    let mut output = String::new();

    // Collect all field names across all records
    let mut all_fields: Vec<String> = Vec::new();
    for record in &doc.records {
        for key in record.fields.keys() {
            if !all_fields.contains(key) {
                all_fields.push(key.clone());
            }
        }
    }
    all_fields.sort();

    // Write header
    write!(output, "name").unwrap();
    for field in &all_fields {
        write!(output, "\t{}", field).unwrap();
    }
    writeln!(output).unwrap();

    // Write rows
    for record in &doc.records {
        write!(output, "{}", record.name).unwrap();
        for field in &all_fields {
            write!(output, "\t").unwrap();
            if let Some(value) = record.fields.get(field) {
                match value {
                    Value::String(s) => write!(output, "{}", s).unwrap(),
                    Value::Number(n) => write!(output, "{}", n).unwrap(),
                    Value::Integer(i) => write!(output, "{}", i).unwrap(),
                    Value::Boolean(b) => write!(output, "{}", b).unwrap(),
                    Value::Reference(r) => write!(output, "{}", r).unwrap(),
                    Value::Array(arr) => {
                        let items: Vec<String> = arr.iter().map(|v| format!("{:?}", v)).collect();
                        write!(output, "[{}]", items.join(",")).unwrap();
                    }
                    Value::Object(_) => write!(output, "{{...}}").unwrap(),
                    Value::Null => {}
                }
            }
        }
        writeln!(output).unwrap();
    }

    output
}
