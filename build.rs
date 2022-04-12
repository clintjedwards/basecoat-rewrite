fn main() {
    tonic_build::configure()
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
}
