use regex::Regex;
use subprocess::{Exec, Redirection};

#[test]
fn echo() {
    let cmd = r#"./target/debug/cluck -s"#;
    let file = r#"
[cmd.echo-1]
exec = "echo hello"

[cmd.echo-2]
exec = "echo world"
    "#;
    let result = Exec::shell(cmd)
        .stdin(file)
        .stdout(Redirection::Pipe)
        .capture()
        .unwrap()
        .stdout_str();

    let pattern = Regex::new(r#"(\[echo-\d\]) (\w+)"#).unwrap();
    assert_eq!(pattern.find_iter(&result).count(), 2)
}
