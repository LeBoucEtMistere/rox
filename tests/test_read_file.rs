use assert_cmd::{prelude::*, Command};

#[test]
fn test_prompt_exits_correctly() {
    let cmd = Command::cargo_bin("rox")
        .expect("Cannot find cargo binary target rox")
        .arg("tests/test.rox")
        .output()
        .expect("rox binary invokation failed");
    cmd.assert().success().stderr("");
}
