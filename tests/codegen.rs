//! Note: if multiple tests that use the same goldenfile are run in parallel, there may be a race condition.

use goldenfile::Mint;

use std::{io::Write, path::PathBuf};

/// Register the goldenfiles for a specific test.
/// This function returns a mint that, when dropped, will compare the "goldenfiles" to the actual output.
/// If they differ, the test will fail.
fn register_goldenfile<P: AsRef<std::path::Path>>(mint: &mut Mint, base_path: P, name: &str) {
    // Register the goldenfile
    let mut golden = mint.new_goldenfile(name).unwrap();

    // Fill the goldenfile with the current content of the file.
    golden
        .write_all(&std::fs::read(base_path.as_ref().join(name)).unwrap())
        .unwrap();
}

/// Teardown the mint.
/// When the mint is dropped, it will compare the goldenfiles to the actual output.
/// But we also want to reset the goldenfiles to their original state.
fn teardown_mint(mint: Mint) {
    // check the goldenfiles, and catch the panic if one occurs so we can reset the goldenfiles afterwards.
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        mint.check_goldenfiles();
    }));

    // reset the goldenfiles to their original state.
    mint.update_goldenfiles();

    // if the check_goldenfiles panicked, panic again.
    if let Err(err) = res {
        std::panic::resume_unwind(err);
    }

    // drop the mint
    drop(mint);
}

#[test]
fn test_const() {
    let base_path = PathBuf::from("test/const");
    let input = base_path.join("rust.rs");
    let output1 = base_path.join("typescript.d.ts");
    let output2 = base_path.join("typescript.ts");

    // Create a new goldenfile mint
    let mut mint = Mint::new(&base_path);
    // Register the goldenfiles
    register_goldenfile(&mut mint, &base_path, "typescript.d.ts");
    register_goldenfile(&mut mint, &base_path, "typescript.ts");

    // Generate the typescript definitions
    tsync::generate_typescript_defs(vec![input.clone()], output1, false, false);
    tsync::generate_typescript_defs(vec![input], output2, false, false);

    // Compare the generated files to the checked-in golden files
    teardown_mint(mint);
}

#[test]
fn test_const_enum_numeric() {
    let base_path = PathBuf::from("test/const_enum_numeric");
    let input = base_path.join("rust.rs");
    let output1 = base_path.join("typescript.d.ts");
    let output2 = base_path.join("typescript.ts");

    // Create a new goldenfile mint
    let mut mint = Mint::new(&base_path);
    // Register the goldenfiles
    register_goldenfile(&mut mint, &base_path, "typescript.d.ts");
    register_goldenfile(&mut mint, &base_path, "typescript.ts");

    // Generate the typescript definitions
    tsync::generate_typescript_defs(vec![input.clone()], output1, false, true);
    tsync::generate_typescript_defs(vec![input], output2, false, true);

    // Compare the generated files to the checked-in golden files and reset the goldenfiles
    teardown_mint(mint);
}

#[test]
fn test_directory_input() {
    let base_path = PathBuf::from("test/directory_input");
    let input = base_path.join("directory");
    let output = base_path.join("typescript.d.ts");

    // Create a new goldenfile mint
    let mut mint = Mint::new(&base_path);
    // Register the goldenfiles
    register_goldenfile(&mut mint, &base_path, "typescript.d.ts");

    // Generate the typescript definitions
    tsync::generate_typescript_defs(vec![input], output, false, false);

    // Compare the generated files to the checked-in golden files and reset the goldenfiles
    teardown_mint(mint);
}

#[test]
fn test_doc_comments() {
    let base_path = PathBuf::from("test/doc_comments");
    let input = base_path.join("rust.rs");
    let output1 = base_path.join("typescript.d.ts");
    let output2 = base_path.join("typescript.ts");

    // Create a new goldenfile mint
    let mut mint = Mint::new(&base_path);
    // Register the goldenfiles
    register_goldenfile(&mut mint, &base_path, "typescript.d.ts");
    register_goldenfile(&mut mint, &base_path, "typescript.ts");

    // Generate the typescript definitions
    tsync::generate_typescript_defs(vec![input.clone()], output1, false, false);
    tsync::generate_typescript_defs(vec![input], output2, false, false);

    // Compare the generated files to the checked-in golden files and reset the goldenfiles
    teardown_mint(mint);
}

#[test]
fn test_enum() {
    let base_path = PathBuf::from("test/enum");
    let input = base_path.join("rust.rs");
    let output1 = base_path.join("typescript.d.ts");
    let output2 = base_path.join("typescript.ts");

    // Create a new goldenfile mint
    let mut mint = Mint::new(&base_path);
    // Register the goldenfiles
    register_goldenfile(&mut mint, &base_path, "typescript.d.ts");
    register_goldenfile(&mut mint, &base_path, "typescript.ts");

    // Generate the typescript definitions
    tsync::generate_typescript_defs(vec![input.clone()], output1, false, false);
    tsync::generate_typescript_defs(vec![input], output2, false, false);

    // Compare the generated files to the checked-in golden files and reset the goldenfiles
    teardown_mint(mint);
}

#[test]
fn test_enum_numeric() {
    let base_path = PathBuf::from("test/enum_numeric");
    let input = base_path.join("rust.rs");
    let output1 = base_path.join("typescript.d.ts");
    let output2 = base_path.join("typescript.ts");

    // Create a new goldenfile mint
    let mut mint = Mint::new(&base_path);
    // Register the goldenfiles
    register_goldenfile(&mut mint, &base_path, "typescript.d.ts");
    register_goldenfile(&mut mint, &base_path, "typescript.ts");

    // Generate the typescript definitions
    tsync::generate_typescript_defs(vec![input.clone()], output1, false, false);
    tsync::generate_typescript_defs(vec![input], output2, false, false);

    // Compare the generated files to the checked-in golden files and reset the goldenfiles
    teardown_mint(mint);
}

#[test]
fn test_generic() {
    let base_path = PathBuf::from("test/generic");
    let input = base_path.join("rust.rs");
    let output1 = base_path.join("typescript.d.ts");
    let output2 = base_path.join("typescript.ts");

    // Create a new goldenfile mint
    let mut mint = Mint::new(&base_path);
    // Register the goldenfiles
    register_goldenfile(&mut mint, &base_path, "typescript.d.ts");
    register_goldenfile(&mut mint, &base_path, "typescript.ts");

    // Generate the typescript definitions
    tsync::generate_typescript_defs(vec![input.clone()], output1, false, false);
    tsync::generate_typescript_defs(vec![input], output2, false, false);

    // Compare the generated files to the checked-in golden files and reset the goldenfiles
    teardown_mint(mint);
}

#[test]
fn test_struct() {
    let base_path = PathBuf::from("test/struct");
    let input = base_path.join("rust.rs");
    let output1 = base_path.join("typescript.d.ts");
    let output2 = base_path.join("typescript.ts");

    // Create a new goldenfile mint
    let mut mint = Mint::new(&base_path);
    // Register the goldenfiles
    register_goldenfile(&mut mint, &base_path, "typescript.d.ts");
    register_goldenfile(&mut mint, &base_path, "typescript.ts");

    // Generate the typescript definitions
    tsync::generate_typescript_defs(vec![input.clone()], output1, false, false);
    tsync::generate_typescript_defs(vec![input], output2, false, false);

    // Compare the generated files to the checked-in golden files and reset the goldenfiles
    teardown_mint(mint);
}

#[test]
fn test_type() {
    let base_path = PathBuf::from("test/type");
    let input = base_path.join("rust.rs");
    let output = base_path.join("typescript.d.ts");

    // Create a new goldenfile mint
    let mut mint = Mint::new(&base_path);
    // Register the goldenfiles
    register_goldenfile(&mut mint, &base_path, "typescript.d.ts");

    // Generate the typescript definitions
    tsync::generate_typescript_defs(vec![input], output, false, false);

    // Compare the generated files to the checked-in golden files and reset the goldenfiles
    teardown_mint(mint);
}
