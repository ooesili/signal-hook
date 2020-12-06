use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;

use cc::Build;

const IN_FILE: &str = "src/constants.c";

fn main() {
    // Hack: we want to compile and run a C program, but we want to do that during the build, not
    // compile it into the result.
    //
    // For that we need to detect the C compiler. But we set the target to the host environment and
    // call it manually.

    println!("cargo:rerun-if-changed={}", IN_FILE);
    let mut out_file: PathBuf = env::var_os("OUT_DIR")
        .expect("Missing the OUT_DIR variable")
        .into();
    out_file.push("constants");
    if cfg!(windows) {
        out_file.set_extension("exe");
    }
    Build::new()
        .target(&env::var("HOST").expect("The HOST is not set"))
        .get_compiler()
        .to_command()
        .args(&[IN_FILE, "-o"])
        .arg(&out_file)
        .status()
        .expect("Failed to compile the extraction tool");

    let cmd = out_file.clone();
    out_file.set_extension("rs");
    let out = File::create(&out_file).expect("Failed to create output file");

    Command::new(&cmd)
        .stdout(out)
        .status()
        .expect("Failed to run the generated tool");
}
