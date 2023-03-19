mod to_typescript;
mod typescript;
pub mod utils;

use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

/// the #[tsync] attribute macro which marks structs and types to be translated into the final typescript definitions file
pub use tsync_macro::tsync;

use crate::to_typescript::ToTypescript;

/// macro to check from an syn::Item most of them have ident attribs
/// that is the one we want to print but not sure!
macro_rules! check_tsync {
    ($x: ident, in: $y: tt, $z: tt, $debug: ident) => {
        let has_tsync_attribute = has_tsync_attribute(&$x.attrs);
        if $debug {
            if has_tsync_attribute {
                println!("Encountered #[tsync] {}: {}", $y, $x.ident.to_string());
            } else {
                println!("Encountered non-tsync {}: {}", $y, $x.ident.to_string());
            }
        }

        if has_tsync_attribute {
            $z
        }
    };
}

pub struct BuildState /*<'a>*/ {
    pub types: String,
    pub unprocessed_files: Vec<PathBuf>,
    // pub ignore_file_config: Option<gitignore::File<'a>>,
}

// fn should_ignore_file(ignore_file: &gitignore::File, entry: &DirEntry) -> bool {
//     let path = entry.path();

//     ignore_file.is_excluded(&path).unwrap_or(false)
// }

fn has_tsync_attribute(attributes: &[syn::Attribute]) -> bool {
    utils::has_attribute("tsync", attributes)
}

impl BuildState {
    fn write_comments(&mut self, comments: &Vec<String>, indentation_amount: i8) {
        let indentation = utils::build_indentation(indentation_amount);
        match comments.len() {
            0 => (),
            1 => self
                .types
                .push_str(&format!("{}/** {} */\n", indentation, &comments[0])),
            _ => {
                self.types.push_str(&format!("{}/**\n", indentation));
                for comment in comments {
                    self.types
                        .push_str(&format!("{} * {}\n", indentation, &comment))
                }
                self.types.push_str(&format!("{} */\n", indentation))
            }
        }
    }
}

fn process_rust_file(
    debug: bool,
    input_path: PathBuf,
    state: &mut BuildState,
    uses_typeinterface: bool,
) {
    if debug {
        dbg!(uses_typeinterface);

        println!(
            "processing rust file: {:?}",
            input_path.clone().into_os_string().into_string().unwrap()
        );
    }

    let file = File::open(&input_path);

    if file.is_err() {
        state.unprocessed_files.push(input_path);
        return;
    }

    let mut file = file.unwrap();

    let mut src = String::new();
    if file.read_to_string(&mut src).is_err() {
        state.unprocessed_files.push(input_path);
        return;
    }

    let syntax = syn::parse_file(&src);

    if syntax.is_err() {
        state.unprocessed_files.push(input_path);
        return;
    }

    let syntax = syntax.unwrap();

    for item in syntax.items {
        match item {
            syn::Item::Const(exported_const) => {
                check_tsync!(exported_const, in: "const", {
                    exported_const.convert_to_ts(state, debug, uses_typeinterface);
                }, debug);
            }
            syn::Item::Struct(exported_struct) => {
                check_tsync!(exported_struct, in: "struct", {
                    exported_struct.convert_to_ts(state, debug, uses_typeinterface);
                }, debug);
            }
            syn::Item::Enum(exported_enum) => {
                check_tsync!(exported_enum, in: "enum", {
                    exported_enum.convert_to_ts(state, debug, uses_typeinterface);
                }, debug);
            }
            syn::Item::Type(exported_type) => {
                check_tsync!(exported_type, in: "type", {
                    exported_type.convert_to_ts(state, debug, uses_typeinterface);
                }, debug);
            }
            _ => {}
        }
    }
}

pub fn generate_typescript_defs(input: Vec<PathBuf>, output: PathBuf, debug: bool) {
    let uses_typeinterface = output
        .as_os_str()
        .to_str()
        .map(|x| x.ends_with(".d.ts"))
        .unwrap_or(true);

    let mut state: BuildState = BuildState {
        types: String::new(),
        unprocessed_files: Vec::<PathBuf>::new(),
        // ignore_file_config: if args.clone().use_ignore_file.is_some() {
        //     match gitignore::File::new(&args.use_ignore_file.unwrap()) {
        //         Ok(gitignore) => Some(gitignore),
        //         Err(err) => {
        //             if args.debug {
        //                 println!("Error: failed to use ignore file! {:#?}", err);
        //             }
        //             None
        //         }
        //     }
        // } else {
        //     None
        // },
    };

    state
        .types
        .push_str("/* This file is generated and managed by tsync */\n");

    for input_path in input {
        if !input_path.exists() {
            if debug {
                println!("Path `{:#?}` does not exist", input_path);
            }

            state.unprocessed_files.push(input_path);
            continue;
        }

        if input_path.is_dir() {
            for entry in WalkDir::new(input_path.clone()).sort_by_file_name() {
                match entry {
                    Ok(dir_entry) => {
                        let path = dir_entry.into_path();

                        // skip dir files because they're going to be recursively crawled by WalkDir
                        if !path.is_dir() {
                            // make sure it is a rust file
                            let extension = path.extension();
                            if extension.is_some() && extension.unwrap().eq_ignore_ascii_case("rs")
                            {
                                process_rust_file(debug, path, &mut state, uses_typeinterface);
                            } else if debug {
                                println!("Encountered non-rust file `{:#?}`", path);
                            }
                        } else if debug {
                            println!("Encountered directory `{:#?}`", path);
                        }
                    }
                    Err(_) => {
                        println!(
                            "An error occurred whilst walking directory `{:#?}`...",
                            input_path.clone()
                        );
                        continue;
                    }
                }
            }
        } else {
            process_rust_file(debug, input_path, &mut state, uses_typeinterface);
        }
    }

    if debug {
        println!("======================================");
        println!("FINAL FILE:");
        println!("======================================");
        println!("{}", state.types);
        println!("======================================");
        println!("Note: Nothing is written in debug mode");
        println!("======================================");
    } else {
        // Verify that the output file either doesn't exists or has been generated by tsync.
        let original_file_path = Path::new(&output);
        if original_file_path.exists() {
            if !original_file_path.is_file() {
                panic!("Specified output path is a directory but must be a file.")
            }
            let original_file = File::open(original_file_path).expect("Couldn't open output file");
            let mut buffer = BufReader::new(original_file);

            let mut first_line = String::new();

            buffer
                .read_line(&mut first_line)
                .expect("Unable to read line");

            if first_line.trim() != "/* This file is generated and managed by tsync */" {
                panic!("Aborting: specified output file exists but doesn't have \"/* This file is generated and managed by tsync */\" as the first line.")
            }
        }

        let mut file: File = File::create(&output).expect("Unable to write to file");
        match file.write_all(state.types.as_bytes()) {
            Ok(_) => println!("Successfully generated typescript types, see {:#?}", output),
            Err(_) => println!("Failed to generate types, an error occurred."),
        }
    }

    if !state.unprocessed_files.is_empty() {
        println!("Could not parse the following files:");
    }

    for unprocessed_file in state.unprocessed_files {
        println!("• {:#?}", unprocessed_file);
    }
}
