mod typescript;

use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use syn::{GenericParam, ItemStruct};
use tsync_macro;
use walkdir::WalkDir;
use crate::typescript::convert_type;

/// the #[tsync] attribute macro which marks structs and types to be translated into the final typescript definitions file
pub use tsync_macro::tsync;

struct BuildState /*<'a>*/ {
    pub types: String,
    pub unprocessed_files: Vec<PathBuf>,
    // pub ignore_file_config: Option<gitignore::File<'a>>,
}

// fn should_ignore_file(ignore_file: &gitignore::File, entry: &DirEntry) -> bool {
//     let path = entry.path();

//     ignore_file.is_excluded(&path).unwrap_or(false)
// }

fn has_tsync_attribute(attributes: &Vec<syn::Attribute>) -> bool {
    attributes.iter().any(|attr| {
        attr.path
            .segments
            .iter()
            .any(|segment| segment.ident.to_string() == "tsync")
    })
}

fn get_comments(attributes: Vec<syn::Attribute>) -> Vec<String> {
    let mut comments: Vec<String> = vec![];

    for attribute in attributes {
        let mut is_doc = false;
        for segment in attribute.path.segments {
            if segment.ident.to_string() == "doc" {
                is_doc = true;
                break;
            }
        }

        if is_doc {
            for token in attribute.tokens {
                match token {
                    syn::__private::quote::__private::TokenTree::Literal(comment) => {
                        let comment = comment.to_string();
                        let comment = comment[1..comment.len() - 1].trim();
                        comments.push(comment.to_string());
                    }
                    _ => { /* Do nothing */ }
                }
            }
        }
    }

    comments
}

fn build_indentation(indentation_amount: i8) -> String {
    let mut indent = "".to_string();
    for _ in 0..indentation_amount {
        indent.push(' ');
    }
    indent
}

fn write_comments(state: &mut BuildState, comments: &Vec<String>, indentation_amount: i8) {
    let indentation = build_indentation(indentation_amount);
    match comments.len() {
        0 => (),
        1 => state.types.push_str(&format!("{}/** {} */\n", indentation, &comments[0])),
        _ => {
            state.types.push_str(&format!("{}/**\n", indentation));
            for comment in comments {
                state.types.push_str(&format!("{} * {}\n", indentation, &comment))
            }
            state.types.push_str(&format!("{} */\n", indentation))
        }
    }
}

fn extract_struct_generics(s: ItemStruct) -> String {
    let mut generic_params: Vec<String> = vec![];

    for generic_param in s.generics.params {
        match generic_param {
            GenericParam::Type(ty) => {
                generic_params.push(ty.ident.to_string())
            },
            _ => {}
        }
    }

    if generic_params.len() == 0 {
        "".to_string()
    } else {
        format!("<{list}>", list = generic_params.join(", "))
    }
}

fn process_rust_file(debug: bool, input_path: PathBuf, state: &mut BuildState) {
    if debug {
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
            syn::Item::Struct(exported_struct) => {
                let has_tsync_attribute = has_tsync_attribute(&exported_struct.attrs);

                if debug {
                    if has_tsync_attribute {
                        println!(
                            "Encountered #[tsync] struct: {}",
                            exported_struct.ident.to_string()
                        );
                    } else {
                        println!(
                            "Encountered non-tsync struct: {}",
                            exported_struct.ident.to_string()
                        );
                    }
                }

                if has_tsync_attribute {
                    state.types.push('\n');

                    let comments = get_comments(exported_struct.clone().attrs);
                    write_comments(state, &comments, 0);

                    state.types.push_str(&format!(
                        "interface {interface_name}{generics} {{\n",
                        interface_name = exported_struct.clone().ident.to_string(),
                        generics = extract_struct_generics(exported_struct.clone())
                    ));
                    for field in exported_struct.fields {
                        let comments = get_comments(field.attrs);
                        write_comments(state, &comments, 2);
                        let field_name = field.ident.unwrap().to_string();
                        let field_type = convert_type(&field.ty);
                        state.types.push_str(&format!(
                            "  {field_name}{optional_parameter_token}: {field_type}\n",
                            field_name = field_name,
                            optional_parameter_token = if field_type.is_optional { "?" } else { "" },
                            field_type = field_type.ts_type
                        ));
                    }
                    state.types.push_str("}");

                    state.types.push('\n');
                }
            }
            syn::Item::Type(exported_type) => {
                let has_tsync_attribute = has_tsync_attribute(&exported_type.attrs);

                if debug {
                    if has_tsync_attribute {
                        println!(
                            "Encountered #[tsync] type: {}",
                            exported_type.ident.to_string()
                        );
                    } else {
                        println!(
                            "Encountered non-tsync type: {}",
                            exported_type.ident.to_string()
                        );
                    }
                }

                if has_tsync_attribute {
                    state.types.push_str("\n");

                    let name = exported_type.ident.to_string();
                    let ty = convert_type(&exported_type.ty);
                    let comments = get_comments(exported_type.attrs);
                    write_comments(state, &comments, 0);
                    state
                        .types
                        .push_str(format!("type {name} = {ty}", name = name, ty = ty.ts_type).as_str());

                    state.types.push_str("\n");
                }
            }
            _ => {}
        }
    }
}

pub fn generate_typescript_defs(input: Vec<PathBuf>, output: PathBuf, debug: bool) {
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
                                process_rust_file(debug, path, &mut state);
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
            process_rust_file(debug, input_path, &mut state);
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
            Ok(_) => println!(
                "Successfully generated typescript types, see {:#?}", output
            ),
            Err(_) => println!("Failed to generate types, an error occurred."),
        }
    }

    if state.unprocessed_files.len() > 0 {
        println!("Could not parse the following files:");
    }

    for unprocessed_file in state.unprocessed_files {
        println!("• {:#?}", unprocessed_file);
    }
}
