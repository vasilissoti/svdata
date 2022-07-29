use anyhow::Error;
use clap::Parser;
use enquote;
use serde_json;
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::{env, process};
use sv_parser::{parse_sv, Define, DefineText, NodeEvent, RefNode, SyntaxTree};
use svdata::structures::SvData;
use svdata::sv_module::{module_declaration_ansi, module_declaration_nonansi};
use verilog_filelist_parser; // DBG

// Clap is used for accepting arguments through command prompt

#[derive(Debug, Parser)]
#[clap(name = "svdata")]
#[clap(long_version(option_env!("LONG_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))))]
pub struct Opt {
    /// Source file
    #[clap(required_unless_present_any = &["filelist"])]
    pub files: Vec<PathBuf>,

    /// File list
    #[clap(short = 'f', long = "filelist", conflicts_with = "files")]
    pub filelist: Vec<PathBuf>,

    /// Define
    #[clap(
        short = 'd',
        long = "define",
        multiple_occurrences = true,
        number_of_values = 1
    )]
    pub defines: Vec<String>,

    /// Include path
    #[clap(
        short = 'i',
        long = "include",
        multiple_occurrences = true,
        number_of_values = 1
    )]
    pub includes: Vec<PathBuf>,

    /// Ignore any include
    #[clap(long = "ignore-include")]
    pub ignore_include: bool,

    /// Suppress description on STDOUT
    #[clap(short = 's', long = "silent")]
    pub silent: bool,

    /// Write output to JSON file
    #[clap(long = "json")]
    pub json: Option<PathBuf>,

    /// Write output to YAML file
    #[clap(long = "yaml")]
    pub yaml: Option<PathBuf>,
}

#[cfg_attr(tarpaulin, skip)]
pub fn main() {
    let opt = Parser::parse(); // This is from clap
    let exit_code = match run_opt(&opt) {
        Ok(_) => 0,
        Err(_) => 1,
    };

    process::exit(exit_code);
}

#[cfg_attr(tarpaulin, skip)]
pub fn run_opt(opt: &Opt) -> Result<SvData, Error> {
    let mut defines = HashMap::new();
    for define in &opt.defines {
        let mut define = define.splitn(2, '=');
        let ident = String::from(define.next().unwrap());
        let text = if let Some(x) = define.next() {
            let x = enquote::unescape(x, None)?;
            Some(DefineText::new(x, None))
        } else {
            None
        };
        let define = Define::new(ident.clone(), vec![], text);
        defines.insert(ident, Some(define));
    }

    let (files, includes) = if !opt.filelist.is_empty() {
        let mut files = opt.files.clone();
        let mut includes = opt.includes.clone();

        for filelist in &opt.filelist {
            let (mut f, mut i, d) = parse_filelist(filelist)?;
            files.append(&mut f);
            includes.append(&mut i);
            for (k, v) in d {
                defines.insert(k, v);
            }
        }

        (files, includes)
    } else {
        (opt.files.clone(), opt.includes.clone())
    };

    let mut svdata = SvData {
        modules: Vec::new(),
        packages: Vec::new(),
    };

    for path in &files {
        match parse_sv(
            &path,
            &defines,
            &includes,
            opt.ignore_include.clone(),
            false,
        ) {
            Ok((syntax_tree, new_defines)) => {
                sv_to_structure(
                    &syntax_tree,
                    &path.to_string_lossy().into_owned(),
                    &mut svdata,
                );
                defines = new_defines;
            }
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "failed to parse '{}'",
                    path.to_string_lossy()
                ))
            }
        }
    }

    if !opt.silent.clone() {
        println!("{}", svdata);
    }

    if let Some(path) = &opt.json {
        let s: String = serde_json::to_string_pretty(&svdata).unwrap();
        let f = Path::new(path);
        let f = File::create(f);
        let mut f = BufWriter::new(f.unwrap());
        write!(f, "{}", s).unwrap();
    }

    if let Some(path) = &opt.yaml {
        let s: String = serde_yaml::to_string(&svdata).unwrap();
        let f = Path::new(path);
        let f = File::create(f);
        let mut f = BufWriter::new(f.unwrap());
        write!(f, "{}", s).unwrap();
    }

    Ok(svdata)
}

// In case that the system verilog files are given in the format of a filelist

#[cfg_attr(tarpaulin, skip)]
fn parse_filelist(
    path: &Path,
) -> Result<(Vec<PathBuf>, Vec<PathBuf>, HashMap<String, Option<Define>>), Error> {
    let filelist = match verilog_filelist_parser::parse_file(path) {
        Ok(f) => f,
        Err(_) => {
            return Err(anyhow::anyhow!(
                "failed to open '{}'",
                path.to_string_lossy()
            ))
        }
    };
    let mut defines = HashMap::new();
    for (d, t) in filelist.defines {
        match t {
            Some(t) => {
                let define_text = DefineText::new(String::from(&t[1..]), None);
                let define = Define::new(String::from(&d), vec![], Some(define_text));
                defines.insert(String::from(&d), Some(define));
            }
            None => {
                defines.insert(String::from(&d), None);
            }
        }
    }

    Ok((filelist.files, filelist.incdirs, defines))
}

fn sv_to_structure(syntax_tree: &SyntaxTree, filepath: &str, svdata: &mut SvData) -> () {
    for event in syntax_tree.into_iter().event() {
        let enter_not_leave = match event {
            NodeEvent::Enter(_) => true,
            NodeEvent::Leave(_) => false,
        };

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(x) => x,
        };

        if enter_not_leave {
            match node {
                RefNode::ModuleDeclarationAnsi(_) => {
                    let d = module_declaration_ansi(node, &syntax_tree, filepath);
                    svdata.modules.push(d.clone());
                }
                RefNode::ModuleDeclarationNonansi(_) => {
                    let _d = module_declaration_nonansi(node, &syntax_tree, filepath);
                }
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::{BufReader, Read};

    fn check_semantics(name: &str) {
        let in_sv = Path::new("testcases")
            .join("semantics")
            .join("sv")
            .join(format!("{}.sv", name));

        let out_dir = env::var("OUT_DIR").unwrap();
        let out_json = Path::new(&out_dir)
            .join("testcases")
            .join("semantics")
            .join("json")
            .join(format!("{}.json", name));
        fs::create_dir_all(out_json.parent().unwrap()).unwrap();
        let out_yaml = Path::new(&out_dir)
            .join("testcases")
            .join("semantics")
            .join("yaml")
            .join(format!("{}.yaml", name));
        fs::create_dir_all(out_yaml.parent().unwrap()).unwrap();

        let mut args = vec!["svdata"];
        args.push(in_sv.to_str().unwrap());
        args.push("--json");
        args.push(out_json.to_str().unwrap());
        args.push("--yaml");
        args.push(out_yaml.to_str().unwrap());
        let opt = Opt::parse_from(args.iter());

        let svdata = run_opt(&opt).unwrap();

        // Write actual display to file for manual inspection.
        let actual_string: String = format!("{}", svdata.clone());
        let out_display = Path::new(&out_dir)
            .join("testcases")
            .join("semantics")
            .join("display")
            .join(format!("{}.txt", name));
        fs::create_dir_all(out_display.parent().unwrap()).unwrap();
        let a = File::create(out_display);
        let mut a = BufWriter::new(a.unwrap());
        write!(a, "{}", actual_string).unwrap();

        // Check display against reference.
        let in_display = Path::new("testcases")
            .join("semantics")
            .join("display")
            .join(format!("{}.txt", name));
        let e = File::open(in_display).unwrap();
        let mut e = BufReader::new(e);
        let mut expected_string: String = String::new();
        e.read_to_string(&mut expected_string).unwrap();
        assert_eq!(expected_string, actual_string);

        // Check JSON against reference.
        let in_json = Path::new("testcases")
            .join("semantics")
            .join("json")
            .join(format!("{}.json", name));
        let e = File::open(in_json).unwrap();
        let e = BufReader::new(e);
        let expected_json_value: serde_json::Value = serde_json::from_reader(e).unwrap();
        let s: String = serde_json::to_string_pretty(&svdata.clone()).unwrap();
        let actual_json_value: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(expected_json_value, actual_json_value);

        // Check YAML against reference.
        let in_yaml = Path::new("testcases")
            .join("semantics")
            .join("yaml")
            .join(format!("{}.yaml", name));
        let e = File::open(in_yaml).unwrap();
        let e = BufReader::new(e);
        let expected_yaml_value: serde_yaml::Value = serde_yaml::from_reader(e).unwrap();
        let s: String = serde_yaml::to_string(&svdata.clone()).unwrap();
        let actual_yaml_value: serde_yaml::Value = serde_yaml::from_str(&s).unwrap();
        assert_eq!(expected_yaml_value, actual_yaml_value);
    }

    fn check_primaryliterals(name: &str, actual_string: String) {
        let out_dir = env::var("OUT_DIR").unwrap();

        // Write actual display to file for manual inspection.
        let out_display = Path::new(&out_dir)
            .join("testcases")
            .join("primaryliterals")
            .join("integral")
            .join("display")
            .join(format!("{}.txt", name));
        fs::create_dir_all(out_display.parent().unwrap()).unwrap();
        let a = File::create(out_display);
        let mut a = BufWriter::new(a.unwrap());
        write!(a, "{}", actual_string).unwrap();

        // Check display against reference.
        let in_display = Path::new("testcases")
            .join("primaryliterals")
            .join("integral")
            .join("display")
            .join(format!("{}.txt", name));
        let e = File::open(in_display).unwrap();
        let mut e = BufReader::new(e);
        let mut expected_string: String = String::new();
        e.read_to_string(&mut expected_string).unwrap();
        assert_eq!(expected_string, actual_string);
    }

    include!(concat!(env!("OUT_DIR"), "/tests.rs"));
}
