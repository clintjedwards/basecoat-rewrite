use std::{env, path::PathBuf, process::Command, time::Instant};

fn get_build_commit() -> String {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap()
}

fn build_frontend_assets() {
    let start = Instant::now();

    let profile = env::var("PROFILE").unwrap();

    if profile == "release" {
        Command::new("npm")
            .args(&["run", "--prefix", "./src/frontend/rebuild-css"])
            .output()
            .unwrap();
        Command::new("npm")
            .args(&["run", "--prefix", "./src/frontend/build"])
            .output()
            .unwrap();
    } else {
        Command::new("npm")
            .args(&["run", "--prefix", "./src/frontend/rebuild-css"])
            .output()
            .unwrap();
        Command::new("npm")
            .args(&["run", "--prefix", "./src/frontend/dev"])
            .output()
            .unwrap();
    }
    println!("Frontend assets built in: {:?}", start.elapsed());
}

fn main() {
    // Build protobuf files.
    tonic_build::configure()
        .file_descriptor_set_path(
            PathBuf::from(env::var("OUT_DIR").unwrap()).join("reflection.bin"),
        )
        .out_dir("src/proto")
        .compile(
            &[
                "src/proto/basecoat.proto",
                "src/proto/basecoat_transport.proto",
                "src/proto/basecoat_message.proto",
            ],
            &["src/proto"],
        )
        .expect("failed compiling protos");

    // Build frontend assets so we can embed them.
    build_frontend_assets();

    // Set binary specific compile time variables.
    println!("cargo:rustc-env=BUILD_SEMVER={}", env!("CARGO_PKG_VERSION"));
    println!("cargo:rustc-env=BUILD_COMMIT={}", get_build_commit());
}
