use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../model/pokemon.smithy");
    Command::new("../gradlew")
        .arg("--project-dir")
        .arg("../")
        .arg("clean")
        .status()
        .unwrap();

    let cmd = Command::new("../gradlew")
        .arg("--project-dir")
        .arg("../")
        .arg("assemble")
        .output()
        .unwrap();

    let out = String::from_utf8_lossy(&cmd.stdout);
    let err = String::from_utf8_lossy(&cmd.stderr);
    if cmd.status.success() {
        println!("cargo:warning=smithy-rs codegen succeeded");
    } else {
        panic!(
            "Failed to build Smithy model, output: {}, error: {}",
            out, err
        );
    }
}
