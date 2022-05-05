use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

fn main(){

    let out_dir = env::var("OUT_DIR").unwrap();


    let mut entries = Vec::new();
    for entry in WalkDir::new("testcases_display_format/expected") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let file_name = String::from(entry.path().file_stem().unwrap().to_string_lossy());
            entries.push(file_name);
        }
    }

    
    // -------------------------------------------------------------------------------------------------
    // Output 'test.rs'
    // -------------------------------------------------------------------------------------------------

    let out_test = Path::new(&out_dir).join("test_display_format.rs");
    let mut out_test = File::create(&out_test).unwrap();

    for file_name in &entries {

        let _ = write!(out_test, "#[test]\n");
        let _ = write!(out_test, "fn test_{}() {{\n", file_name);
        let _ = write!(out_test, "    test_display_format(\"{}\");\n", file_name);
        let _ = write!(out_test, "}}\n");
    
    }
}