use std::path::Path;
use std::process::Command;

fn main() {
    // Run this once during development to create the dump file
    let mut builder = syntect::parsing::SyntaxSetBuilder::new();
    builder
        .add_from_folder(std::path::Path::new("kasl_syntax"), true)
        .expect("Could not load syntax from kasl_syntax directory");
    let syntax_set = builder.build();

    // Save the compiled state to a file
    syntect::dumps::dump_to_file(&syntax_set, "kasl_syntax/kasl_syntax.pack")
        .expect("Could not save syntax set to kasl_syntax/kasl_syntax.pack");
}
