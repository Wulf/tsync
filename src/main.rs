extern crate syn;

use std::io::prelude::*;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Debug, StructOpt)]
#[structopt(about = DESCRIPTION, after_help = "This command helps generate type information for other languages. Currently, only typescript is supported.")]
struct Args {
    /// Activate debug mode
    #[structopt(long, help = "Dry-run, prints to stdout", short = "d", long = "debug")]
    debug: bool,

    /// Output format
    #[structopt(short = "f", long = "format", default_value = "typescript", help = "Currently, only \"typescript\" is accepted and is the default")]
    format: String,

    /// Input file
    #[structopt(short = "i", long = "input", help = "Required; rust file(s) to read type information from", required = true)]
    input: Vec<PathBuf>,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str), short = "o", long = "output", help = "Required; file to write generated types to")]
    output: PathBuf,
}

fn to_typsecript_type(gen_ty: &syn::GenericArgument) -> String {
    match gen_ty {
        syn::GenericArgument::Type(ty) => to_typescript_type(ty),
        _ => "unknown".to_string()
    }
}

fn to_typescript_type(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Reference(p) => {
            to_typescript_type(&*p.elem)
        },
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
                    syn::PathArguments::Parenthesized(parenthesized_argument) => format!("{:?}", parenthesized_argument),
                    syn::PathArguments::AngleBracketed(anglebracketed_argument) => format!("{} | undefined", to_typsecript_type(anglebracketed_argument.args.first().unwrap())),
                    _ => "unknown".to_string()
                },
                "Vec" => match arguments {
                    syn::PathArguments::Parenthesized(parenthesized_argument) => format!("{:?}", parenthesized_argument),
                    syn::PathArguments::AngleBracketed(anglebracketed_argument) => format!("Array<{}>", to_typsecript_type(anglebracketed_argument.args.first().unwrap())),
                    _ => "unknown".to_string()
                },
                _ => identifier.to_string()
            }
        },
        _ => "unknown".to_string()
    }
}

fn has_tsync_attribute(attributes: &Vec<syn::Attribute>) -> bool {
    attributes.iter().any(
        |attr| attr.path.segments.iter().any(
            |segment| segment.ident.to_string() == "tsync"
        )
    )
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
                        let comment = comment[1..comment.len()-1].trim();
                        comments.push(comment.to_string());
                    },
                    _ => { /* Do nothing */}
                }
            }
        }
    }
    
    comments
}

fn main() {
    let args: Args = Args::from_args();
    let mut unprocessed_files: Vec<PathBuf> = Vec::<PathBuf>::new();
    let mut types: String = String::new();

    types.push_str("/* This file is generated and managed by tsync */\n");

    for input_path in args.input {
        if args.debug {
            println!("processing rust file: {:?}", input_path.clone().into_os_string().into_string().unwrap());
        }

        let file = File::open(&input_path);

        if file.is_err() {
            unprocessed_files.push(input_path);
            continue;
        }

        let mut file = file.unwrap();
        
        let mut src = String::new();
        if file.read_to_string(&mut src).is_err() {
            unprocessed_files.push(input_path);
            continue;
        }

        let syntax = syn::parse_file(&src);

        if syntax.is_err() {
            unprocessed_files.push(input_path);
            continue;
        }

        let syntax = syntax.unwrap();
        
        for item in syntax.items {
            match item {
                syn::Item::Struct(exported_struct) => {
                    let has_tsync_attribute = has_tsync_attribute(&exported_struct.attrs);

                    if args.debug {
                        if has_tsync_attribute {
                            println!("Encountered #[tsync] struct: {}", exported_struct.ident.to_string());
                        } else {
                            println!("Encountered non-tsync struct: {}", exported_struct.ident.to_string());
                        }
                    }

                    if has_tsync_attribute {
                        types.push('\n');

                        let comments = get_comments(exported_struct.attrs);
                        if comments.len() > 0 {
                            for comment in comments {
                                types.push_str(&format!("// {}\n", &comment))
                            }
                        }

                        types.push_str(&format!("interface {interface_name} {{\n", interface_name=exported_struct.ident.to_string()));
                        for field in exported_struct.fields {
                            let comments = get_comments(field.attrs);
                            if comments.len() > 0 {
                                for comment in comments {
                                    types.push_str(&format!("  // {}\n", &comment))
                                }
                            }   
                            let field_name = field.ident.unwrap().to_string();
                            let field_type: String = to_typescript_type(&field.ty);
                            types.push_str(&format!("  {field_name}: {field_type}\n", field_name=field_name, field_type=field_type));
                        }
                        types.push_str("}");

                        types.push('\n');
                    }
                },
                syn::Item::Type(exported_type) => {
                    let has_tsync_attribute = has_tsync_attribute(&exported_type.attrs);

                    if args.debug {
                        if has_tsync_attribute {
                            println!("Encountered #[tsync] type: {}", exported_type.ident.to_string());
                        } else {
                            println!("Encountered non-tsync type: {}", exported_type.ident.to_string());
                        }
                    }

                    if has_tsync_attribute {
                        types.push_str("\n");

                        let name = exported_type.ident.to_string();
                        let ty: String = to_typescript_type(&exported_type.ty);
                        let comments = get_comments(exported_type.attrs);
                        if comments.len() > 0 {
                            for comment in comments {
                                types.push_str(&format!("// {}\n", &comment))
                            }
                        }
                        types.push_str(format!("type {name} = {ty}",
                            name=name,
                            ty=ty
                        ).as_str());

                        types.push_str("\n");
                    }
                },
                _ => { }
            }
        }
    }
    
    if args.debug {
        println!("======================================");
        println!("FINAL FILE:");
        println!("======================================");
        println!("{}", types);
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

            buffer.read_line(&mut first_line).expect("Unable to read line");

            if first_line.trim() != "/* This file is generated and managed by tsync */" {
                panic!("Aborting: specified output file exists but doesn't have \"/* This file is generated and managed by tsync */\" as the first line.")
            }
        }

        let mut file: File = File::create(&args.output).expect("Unable to write to file");
        match file.write_all(types.as_bytes()) {
            Ok(_) => println!("Successfully generated {} types, see {:#?}", args.format, args.output),
            Err(_) => println!("Failed to generate types, an error occurred.")
        }
    }

    if unprocessed_files.len() > 0 {
        println!("Could not parse the following files:");
    }

    for unprocessed_file in unprocessed_files {
        println!("• {:#?}", file=unprocessed_file);
    }
}