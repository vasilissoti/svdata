use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut entries = Vec::new();
    for entry in WalkDir::new("testcases/sv") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let file_name = String::from(entry.path().file_stem().unwrap().to_string_lossy());
            entries.push(file_name);
        }
    }

    // -------------------------------------------------------------------------------------------------
    // Output 'test.rs'
    // -------------------------------------------------------------------------------------------------

    let t = Path::new(&out_dir).join("tests.rs");
    let mut t = File::create(&t).unwrap();

    for file_name in &entries {
        write!(t, "#[test]\n").unwrap();
        write!(t, "fn test_{}() {{\n", file_name).unwrap();
        write!(t, "    check_outputs(\"{}\");\n", file_name).unwrap();
        write!(t, "}}\n").unwrap();
    }
}
