use assert_cmd::Command;

#[test]
fn converts_to_different_cases() {
    let test_cases = vec![
        ("camel", "hello world", "helloWorld"),
        ("pascal", "hello world", "HelloWorld"),
    ];

    for (case, input, expected) in test_cases {
        let mut cmd = Command::cargo_bin("inflect").expect("Could not run binary.");
        cmd.arg(case).arg(input).assert().success().stdout(expected);
    }
}
