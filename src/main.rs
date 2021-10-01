extern crate syn;

use std::io::prelude::*;

// Add this to Cargo.toml: gitignore = "1.0.7"
// use gitignore; TODO: add flag which can parse and apply .gitignore
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;
use walkdir::WalkDir;

const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Debug, StructOpt, Clone)]
#[structopt(about = DESCRIPTION, after_help = "This command helps generate type information for other languages. Currently, only typescript is supported.")]
struct Args {
    /// Activate debug mode
    #[structopt(long, help = "Dry-run, prints to stdout", short = "d", long = "debug")]
    debug: bool,

    /// Output format
    #[structopt(
        short = "f",
        long = "format",
        default_value = "typescript",
        help = "Currently, only \"typescript\" is accepted and is the default"
    )]
    format: String,

    // #[structopt(
    //     long = "use-ignore-file",
    //     help = "Optionally ignore files with a .gitignore (or similar file); for example: --use-ignore-file=.gitignore"
    // )]
    // use_ignore_file: Option<PathBuf>,
    /// Input file
    #[structopt(
        short = "i",
        long = "input",
        help = "Required; rust file(s) to read type information from",
        required = true
    )]
    input: Vec<PathBuf>,

    /// Output file, stdout if not present
    #[structopt(
        parse(from_os_str),
        short = "o",
        long = "output",
        help = "Required; file to write generated types to"
    )]
    output: PathBuf,
}

fn to_typsecript_type(gen_ty: &syn::GenericArgument) -> String {
    match gen_ty {
        syn::GenericArgument::Type(ty) => to_typescript_type(ty),
        _ => "unknown".to_string(),
    }
}

fn to_typescript_type(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Reference(p) => to_typescript_type(&*p.elem),
        syn::Type::Path(p) => {
            let segment = p.path.segments.last().unwrap();
            let ident = &segment.ident;
            let arguments = &segment.arguments;
            let identifier = ident.to_string();
            match identifier.as_str() {
                "i8" => "number".to_string(),
                "u8" => "number".to_string(),
                "i16" => "number".to_string(),
                "u16" => "number".to_string(),
                "i32" => "number".to_string(),
                "u32" => "number".to_string(),
                "i64" => "number".to_string(),
                "u64" => "number".to_string(),
                "i128" => "number".to_string(),
                "u128" => "number".to_string(),
                "isize" => "number".to_string(),
                "usize" => "number".to_string(),
                "f32" => "number".to_string(),
                "f64" => "number".to_string(),
                "bool" => "boolean".to_string(),
                "char" => "string".to_string(),
                "str" => "string".to_string(),
                "String" => "string".to_string(),
                "NaiveDateTime" => "Date".to_string(),
                "DateTime" => "Date".to_string(),
                "Option" => match arguments {
                    syn::PathArguments::Parenthesized(parenthesized_argument) => {
                        format!("{:?}", parenthesized_argument)
                    }
                    syn::PathArguments::AngleBracketed(anglebracketed_argument) => format!(
                        "{} | undefined",
                        to_typsecript_type(anglebracketed_argument.args.first().unwrap())
                    ),
                    _ => "unknown".to_string(),
                },
                "Vec" => match arguments {
                    syn::PathArguments::Parenthesized(parenthesized_argument) => {
                        format!("{:?}", parenthesized_argument)
                    }
                    syn::PathArguments::AngleBracketed(anglebracketed_argument) => format!(
                        "Array<{}>",
                        to_typsecript_type(anglebracketed_argument.args.first().unwrap())
                    ),
                    _ => "unknown".to_string(),
                },
                _ => identifier.to_string(),
            }
        }
        _ => "unknown".to_string(),
    }
}

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

struct BuildState /*<'a>*/ {
    pub types: String,
    pub unprocessed_files: Vec<PathBuf>,
    // pub ignore_file_config: Option<gitignore::File<'a>>,
}

fn process_rust_file(args: Args, input_path: PathBuf, state: &mut BuildState) {
    if args.debug {
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

                if args.debug {
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

                    let comments = get_comments(exported_struct.attrs);
                    if comments.len() > 0 {
                        for comment in comments {
                            state.types.push_str(&format!("// {}\n", &comment))
                        }
                    }

                    state.types.push_str(&format!(
                        "interface {interface_name} {{\n",
                        interface_name = exported_struct.ident.to_string()
                    ));
                    for field in exported_struct.fields {
                        let comments = get_comments(field.attrs);
                        if comments.len() > 0 {
                            for comment in comments {
                                state.types.push_str(&format!("  // {}\n", &comment))
                            }
                        }
                        let field_name = field.ident.unwrap().to_string();
                        let field_type: String = to_typescript_type(&field.ty);
                        state.types.push_str(&format!(
                            "  {field_name}: {field_type}\n",
                            field_name = field_name,
                            field_type = field_type
                        ));
                    }
                    state.types.push_str("}");

                    state.types.push('\n');
                }
            }
            syn::Item::Type(exported_type) => {
                let has_tsync_attribute = has_tsync_attribute(&exported_type.attrs);

                if args.debug {
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
                    let ty: String = to_typescript_type(&exported_type.ty);
                    let comments = get_comments(exported_type.attrs);
                    if comments.len() > 0 {
                        for comment in comments {
                            state.types.push_str(&format!("// {}\n", &comment))
                        }
                    }
                    state
                        .types
                        .push_str(format!("type {name} = {ty}", name = name, ty = ty).as_str());

                    state.types.push_str("\n");
                }
            }
            _ => {}
        }
    }
}

// fn should_ignore_file(ignore_file: &gitignore::File, entry: &DirEntry) -> bool {
//     let path = entry.path();

//     ignore_file.is_excluded(&path).unwrap_or(false)
// }

fn main() {
    let args: Args = Args::from_args();
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

    for input_path in args.clone().input {
        if !input_path.exists() {
            if args.debug {
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
                        if !path.is_dir() {
                            // skip dir files because they're going to be recursively crawled by WalkDir
                            process_rust_file(args.clone(), path, &mut state);
                        } else if args.debug {
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
            process_rust_file(args.clone(), input_path, &mut state);
        }
    }

    if args.debug {
        println!("======================================");
        println!("FINAL FILE:");
        println!("======================================");
        println!("{}", state.types);
        println!("======================================");
        println!("Note: Nothing is written in debug mode");
        println!("======================================");
    } else {
        // Verify that the output file either doesn't exists or has been generated by tsync.
        let original_file_path = Path::new(&args.output);
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

        let mut file: File = File::create(&args.output).expect("Unable to write to file");
        match file.write_all(state.types.as_bytes()) {
            Ok(_) => println!(
                "Successfully generated {} types, see {:#?}",
                args.format, args.output
            ),
            Err(_) => println!("Failed to generate types, an error occurred."),
        }
    }

    if state.unprocessed_files.len() > 0 {
        println!("Could not parse the following files:");
    }

    for unprocessed_file in state.unprocessed_files {
        println!("• {:#?}", file = unprocessed_file);
    }
}
