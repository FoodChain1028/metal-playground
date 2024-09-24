use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn compile_metal_shaders() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let source_dir = format!("{}/src/shaders", manifest_dir);
    let build_dir = format!("{}/build", manifest_dir);

    // Tell Cargo to rerun the build script if the shaders directory changes
    println!("cargo:rerun-if-changed={}", source_dir);

    // Create the build directory if it doesn't exist
    if !Path::new(&build_dir).exists() {
        fs::create_dir_all(&build_dir).expect("Failed to create build directory");
    }

    let input = format!("{}/shaders.metal", source_dir);
    let output = format!("{}/shaders.metallib", build_dir);

    println!("cargo:warning=Compiling '{}' to '{}'", input, output);

    let cmd = Command::new("xcrun")
        .args(["-sdk", "macosx", "metal"])
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .spawn()
        .expect("Failed to spawn process");

    let res = cmd.wait_with_output().expect("Command waiting failed");

    if !res.status.success() {
        panic!("Failed to compile Metal shader: {}", input);
    }

    // Optionally, print a success message
    println!("cargo:warning=Successfully compiled '{}'", output);
}

fn main() {
    compile_metal_shaders();
}
