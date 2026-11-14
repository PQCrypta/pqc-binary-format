//! Every copy of the algorithm registry outside `src/algorithm.rs` is held
//! to it: the C header's constants, the Go binding's identifiers and names,
//! the README's list and counts, and docs/algorithms.md. Each had drifted
//! while kept by hand (a C constant naming 0x0208 for Lattice-Code Hybrid,
//! 0x0206; a header declaring 13 of the 24 constants the library exported;
//! a README and a reference counting 47 and 31 of a different list).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use pqc_binary_format::Algorithm;

fn read(relative: &str) -> String {
    std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(relative))
        .unwrap_or_else(|e| panic!("{relative}: {e}"))
        .replace("\r\n", "\n")
}

/// Letters and digits only, lowercase: Go's `MaxSecurePurePQ` and Rust's
/// `MaxSecurePurePq` are the same name.
fn folded(name: &str) -> String {
    name.chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

fn hex(token: &str) -> u16 {
    u16::from_str_radix(token.trim_start_matches("0x"), 16)
        .unwrap_or_else(|e| panic!("{token}: {e}"))
}

/// Every word in `text` that starts with `prefix`, up to the first
/// character that cannot be in an identifier.
fn identifiers_after<'a>(text: &'a str, prefix: &str) -> Vec<&'a str> {
    text.match_indices(prefix)
        .map(|(at, _)| {
            let rest = &text[at + prefix.len()..];
            let end = rest
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .unwrap_or(rest.len());
            &rest[..end]
        })
        .collect()
}

#[test]
fn the_c_header_declares_every_constant() {
    let header = read("include/pqc_binary_format.h");
    let declared: BTreeSet<String> = identifiers_after(&header, "extern const uint16_t ")
        .into_iter()
        .map(str::to_string)
        .collect();
    let exported: BTreeSet<String> = Algorithm::ALL
        .iter()
        .map(|a| a.c_constant().to_string())
        .collect();
    assert_eq!(
        declared, exported,
        "include/pqc_binary_format.h declares other constants than the library exports"
    );
}

#[test]
fn the_go_binding_carries_every_identifier_and_name() {
    let go = read("bindings/go/format.go");
    // `AlgorithmX uint16 = 0x0000` in the const blocks.
    let mut constants: BTreeMap<String, u16> = BTreeMap::new();
    for line in go.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("Algorithm") else {
            continue;
        };
        let mut words = rest.split_whitespace();
        let (Some(name), Some("uint16"), Some("=")) = (words.next(), words.next(), words.next())
        else {
            continue;
        };
        let value = words.next().expect("a value");
        constants.insert(name.to_string(), hex(value));
    }
    // `AlgorithmX: "Name",` in algorithmNames.
    let mut names: BTreeMap<String, String> = BTreeMap::new();
    for line in go.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("Algorithm") else {
            continue;
        };
        let Some((name, value)) = rest.split_once(':') else {
            continue;
        };
        let value = value.trim().trim_end_matches(',');
        if let Some(value) = value.strip_prefix('"').and_then(|v| v.strip_suffix('"')) {
            names.insert(name.to_string(), value.to_string());
        }
    }
    assert_eq!(
        constants.len(),
        Algorithm::ALL.len(),
        "Go constants: {constants:?}"
    );
    for algo in Algorithm::ALL {
        let (go_name, _) = constants
            .iter()
            .find(|(_, id)| **id == algo.as_id())
            .unwrap_or_else(|| {
                panic!("Go has no constant for {:?} (0x{:04X})", algo, algo.as_id())
            });
        assert_eq!(
            folded(go_name),
            folded(&format!("{algo:?}")),
            "Go's name for 0x{:04X}",
            algo.as_id()
        );
        assert_eq!(
            names.get(go_name).map(String::as_str),
            Some(algo.name()),
            "Go's display name for {go_name}"
        );
    }
}

#[test]
fn the_readme_lists_every_algorithm() {
    let readme = read("README.md");
    let section = &readme[readme
        .find("## 🔐 Supported Algorithms")
        .expect("the section")..];
    let section = &section[..section[3..].find("\n## ").map_or(section.len(), |n| n + 3)];
    for algo in Algorithm::ALL {
        let entry = format!("**{}** (0x{:04X})", algo.name(), algo.as_id());
        assert!(section.contains(&entry), "README's list has no {entry}");
    }
    let listed = section.matches("- **").count();
    assert_eq!(
        listed,
        Algorithm::ALL.len(),
        "README lists {listed} algorithms"
    );
    let count = Algorithm::ALL.len();
    for claim in [
        format!("The format supports {count} cryptographic algorithm identifiers"),
        format!("Works across **{count} cryptographic algorithms**"),
        format!("Full algorithm suite ({count} algorithms)"),
    ] {
        assert!(readme.contains(&claim), "README does not say: {claim}");
    }
}

#[test]
fn the_algorithm_reference_has_a_section_for_each() {
    let doc = read("docs/algorithms.md");
    let headings: Vec<(usize, u16)> = doc
        .lines()
        .filter_map(|line| {
            let rest = line.strip_prefix("### ")?;
            let (number, rest) = rest.split_once(". ")?;
            let id = rest.rsplit_once("(0x")?.1.strip_suffix(')')?;
            Some((number.parse().ok()?, hex(id)))
        })
        .collect();
    let expected: Vec<(usize, u16)> = Algorithm::ALL
        .iter()
        .enumerate()
        .map(|(n, a)| (n + 1, a.as_id()))
        .collect();
    assert_eq!(headings, expected, "docs/algorithms.md's numbered sections");
    assert!(
        doc.contains(&format!(
            "One section for each of the {} algorithms",
            Algorithm::ALL.len()
        )),
        "docs/algorithms.md states another count"
    );
}
