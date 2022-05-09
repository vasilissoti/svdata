use std::env;
use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;
use std::collections::HashMap;
use std::fmt;



fn main(){

    let out_dir = env::var("OUT_DIR").unwrap();


    let mut entries = Vec::new();
    for entry in WalkDir::new("testcases/sv_files") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let file_name = String::from(entry.path().file_stem().unwrap().to_string_lossy());
            entries.push(file_name);
        }
    }

    
    // -------------------------------------------------------------------------------------------------
    // Output 'test.rs'
    // -------------------------------------------------------------------------------------------------
    let mut overall_excluded_files: Vec<String> = Vec::new();
    let mut custom_excluded_files_display: Vec<String> = Vec::new();
    let mut custom_excluded_files_json: Vec<String> = Vec::new();
    let mut custom_excluded_files_yaml: Vec<String> = Vec::new();
    let mut format_settings: HashMap<String, String> = HashMap::new(); // VNotes: keeping the test settings (formats of data tested)

    let settings_file = File::open("testcases/settings.txt");
    let settings_file = BufReader::new(settings_file.unwrap());

    let mut begin_overall_excluding: bool = false;
    let mut begin_display_excluding: bool = false;
    let mut begin_json_excluding: bool = false;
    let mut begin_yaml_excluding: bool = false;

    let mut set_display_format: bool = false;
    let mut set_json_format: bool = false;
    let mut set_yaml_format: bool = false;
    
    for line in settings_file.lines(){
        for word in line.unwrap().split_whitespace(){

            match word {
                
                "[Files_Excluded_Overall_Testing:]" => {
                    begin_overall_excluding = true;
                    continue;
                },

                "[Tested_Formats_To_Run:]" => {
                    begin_overall_excluding = false;
                    continue;
                },

                "Display:" => {
                    set_display_format = true;
                    continue;
                },

                "JSON:" => {
                    set_display_format = false;
                    set_json_format = true;
                    continue;
                },

                "YAML:" => {
                    set_json_format = false;
                    set_yaml_format = true;
                    continue;
                },

                "[Files_Excluded_Display:]" => {
                    set_yaml_format = false;
                    begin_display_excluding =true;
                    continue;
                },

                "[Files_Excluded_JSON:]" => {
                    begin_display_excluding =false;
                    begin_json_excluding =true;
                    continue;
                },

                "[Files_Excluded_YAML:]" => {
                    begin_json_excluding =false;
                    begin_yaml_excluding =true;
                    continue;
                },

                _ => {},
            };

            if begin_overall_excluding {
                overall_excluded_files.push(String::from(word)); 
            }

            else if set_display_format {
                format_settings.insert(String::from("Display"), String::from(word));
            }

            else if set_json_format {
                format_settings.insert(String::from("JSON"), String::from(word));
            }

            else if set_yaml_format {
                format_settings.insert(String::from("YAML"), String::from(word));
            }

            else if begin_display_excluding {
                custom_excluded_files_display.push(String::from(word)); 
            }

            else if begin_json_excluding {
                custom_excluded_files_json.push(String::from(word));
            }

            else if begin_yaml_excluding {
                custom_excluded_files_yaml.push(String::from(word)); 
            }
        }
    }


    let out_test = Path::new(&out_dir).join("tests.rs");
    let mut out_test = File::create(&out_test).unwrap();
    let mut out_test_temp: String = String::new();
    
    for file_name in &entries {

        let mut run_display_test: bool = true;
        let mut run_json_test: bool = true;
        let mut run_yaml_test: bool = true;
            
        if custom_excluded_files_display.contains(file_name) || format_settings.get(&String::from("Display")).unwrap() == "No" {
            run_display_test = false;
        }
        if custom_excluded_files_json.contains(file_name) || format_settings.get(&String::from("JSON")).unwrap() == "No" {
            run_json_test = false;
        }
        if custom_excluded_files_yaml.contains(file_name) || format_settings.get(&String::from("YAML")).unwrap() == "No" {
            run_yaml_test = false;
        }

        if overall_excluded_files.contains(file_name) {
            continue;
        }

        else if run_display_test == false && run_json_test == false && run_yaml_test == false {
            // VNotes: This ensures that even if we haven't placed the filename in the overall excluded vec, if through the custom module or format settings we end up with all three tests disabled we can include it as well in the vector
            overall_excluded_files.push(String::from(file_name));
        }

        else {
            let _ = fmt::write(&mut out_test_temp, format_args!("#[test]\n"));
            let _ = fmt::write(&mut out_test_temp, format_args!("fn test_{}() {{\n", file_name));
            let _ = fmt::write(&mut out_test_temp, format_args!("    tests(\"{}\", {}, {}, {});\n", file_name, run_display_test, run_json_test, run_yaml_test));
            let _ = fmt::write(&mut out_test_temp, format_args!("}}\n"));
        }
    }

    // VNotes: In the upper part of the file point out the excluded files from all the testing formats
    let _ = write!(out_test, "// Overall Excluded Files: \n");
    let _ = write!(out_test, "/* {:?}", overall_excluded_files);
    let _ = writeln!(out_test, " */\n");
    let _ = writeln!(out_test, "{}", out_test_temp);



}