use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../model/pokemon.smithy");
    let cmd = Command::new("../gradlew")
        .arg("--project-dir")
        .arg("../")
        .arg("model::assemble")
        .output()
        .unwrap();

    if ! cmd.status.success() {
        let out = String::from_utf8_lossy(&cmd.stdout);
        let err = String::from_utf8_lossy(&cmd.stderr);
        panic!("Failed to build Smithy model, output: {}, error: {}", out, err);
    }
}
