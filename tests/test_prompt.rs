use assert_cmd::{prelude::*, Command};

#[test]
fn test_prompt_exits_correctly() {
    for exit_word in ["exit", "exit()", "quit", "quit()"] {
        let cmd = Command::cargo_bin("rox")
            .expect("Cannot find cargo binary target rox")
            .write_stdin(format!("\n{}", exit_word))
            .output()
            .expect("rox binary invokation failed");
        cmd.assert().success().stdout("> \n> ").stderr("");
    }
}

#[test]
fn test_prompt_interrupted_correctly() {
    let cmd = Command::cargo_bin("rox")
        .expect("Cannot invoke rox binary")
        .write_stdin([]) // this empty buffer triggers sending an EOF directly
        .output()
        .expect("rox binary invokation failed");
    cmd.assert().success().stdout("> ").stderr("");
}
