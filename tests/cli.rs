use assert_cmd::cargo;
use std::process::Command;

#[test]
fn runs() {
    let mut cmd = Command::new(cargo::cargo_bin!("humble-cli"));
    let output = cmd.output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    let msg = String::from_utf8(output.stderr).expect("failed to convert to String");
    assert!(msg.contains("The missing Humble Bundle CLI"));
}
