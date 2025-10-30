use clap::Parser;
use std::path::PathBuf;

const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Debug, Parser, Clone)]
#[command(about = DESCRIPTION, after_help = "This command helps generate type information for other languages. Currently, only typescript is supported.")]
struct Args {
    /// Activate debug mode
    #[clap(
        short = 'd',
        long,
        alias = "dry-run",
        help = "Dry-run, prints to stdout"
    )]
    debug: bool,

    /// Enable const enums
    #[clap(short, long = "const-enums", help = "Enable generating const enums")]
    enable_const_enums: bool,

    // TODO: add "create-module" functionality (so generated types can be under a specified namespace like Rust.MyType)
    // useModules: bool,

    // TODO: add .gitignore (and other ignore files) parsing functinality
    // Add this to Cargo.toml: gitignore = "1.0.7"
    // use gitignore; TODO: add flag which can parse and apply .gitignore
    // #[clap(
    //     long,
    //     help = "Optionally ignore files with a .gitignore (or similar file); for example: --use-ignore-file=.gitignore"
    // )]
    // use_ignore_file: Option<PathBuf>,
    /// Input file
    #[clap(
        short,
        long,
        help = "Required; rust file(s) to read type information from",
        required = true
    )]
    input: Vec<PathBuf>,

    /// Output file (this is the "<name>.d.ts" that gets generated)
    #[clap(
        short,
        long,
        help = "Required; file to write generated types to",
        required = true
    )]
    output: PathBuf,
}

fn main() {
    let args: Args = Args::parse();

    tsync::generate_typescript_defs(args.input, args.output, args.debug, args.enable_const_enums);
}
