//! Path filtering for asset extraction

use crate::cli::Args;

/// Check if a path matches the configured filters
pub fn matches_filters(path: &str, args: &Args) -> bool {
    // Check excludes first
    for pattern in &args.exclude {
        if glob_match::glob_match(pattern, path) {
            return false;
        }
    }

    // If no positive filters, match all
    if args.select.is_empty() && args.filter.is_empty() && args.ifilter.is_empty() {
        return true;
    }

    // Check select patterns (glob)
    for pattern in &args.select {
        if glob_match::glob_match(pattern, path) {
            return true;
        }
    }

    // Check filter (substring)
    for f in &args.filter {
        if path.contains(f) {
            return true;
        }
    }

    // Check ifilter (case-insensitive substring)
    let path_lower = path.to_lowercase();
    for f in &args.ifilter {
        if path_lower.contains(&f.to_lowercase()) {
            return true;
        }
    }

    false
}
