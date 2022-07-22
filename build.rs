use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::iter::zip;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut entries_semantics = Vec::new();
    for entry in WalkDir::new("testcases/semantics/sv") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let file_name = String::from(entry.path().file_stem().unwrap().to_string_lossy());
            entries_semantics.push(file_name);
        }
    }

    let mut entries_primlits = Vec::new();
    let mut entries_primlits_contents: Vec<Vec<String>> = Vec::new();

    for entry in WalkDir::new("testcases/primaryliterals/integral/rs") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let file_name = String::from(entry.path().file_stem().unwrap().to_string_lossy());
            entries_primlits.push(file_name);

            let file = File::open(entry.path()).unwrap();
            let content = BufReader::new(&file);
            let content_lines: Vec<String> = content.lines().collect::<Result<_, _>>().unwrap();
            entries_primlits_contents.push(content_lines.clone());
        }
    }

    // -------------------------------------------------------------------------------------------------
    // Output 'test.rs'
    // -------------------------------------------------------------------------------------------------

    let t = Path::new(&out_dir).join("tests.rs");
    let mut t = File::create(&t).unwrap();

    writeln!(t, "use svdata::sv_primlit_integral::*;\n").unwrap();

    for file_name in &entries_semantics {
        write!(t, "#[test]\n").unwrap();
        write!(t, "fn test_{}() {{\n", file_name).unwrap();
        write!(t, "    check_semantics(\"{}\");\n", file_name).unwrap();
        write!(t, "}}\n").unwrap();
    }

    let length = entries_primlits.len();
    let mut iter = zip(entries_primlits, entries_primlits_contents);

    for _x in 0..length {
        let (file_name, content) = iter.next().unwrap();

        write!(t, "#[test]\n").unwrap();
        write!(t, "fn test_{}() {{\n", file_name).unwrap();

        for line in &content {
            _ = writeln!(t, "    {}", line.as_str());
        }

        write!(
            t,
            "    check_primaryliterals(\"{}\", actual_string);\n",
            file_name
        )
        .unwrap();
        write!(t, "}}\n").unwrap();
    }
}
