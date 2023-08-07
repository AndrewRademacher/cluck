use regex::Regex;
use subprocess::{Exec, Redirection};

#[test]
fn echo() {
    let cmd = r#"./target/debug/cluck run "echo hello" "echo world" "#;
    let result = Exec::shell(cmd)
        .stdout(Redirection::Pipe)
        .capture()
        .unwrap()
        .stdout_str();

    let pattern = Regex::new(r#"(\[echo \w+\]) (\w+)"#).unwrap();
    assert_eq!(pattern.find_iter(&result).count(), 2)
}
