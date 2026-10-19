use crate::harness::pamtester;

#[test]
fn test_username_example_module() {
    let test_username = "testuser";
    let output = pamtester(
        "username",
        &["session required"],
        Some(test_username),
        "open_session",
        &[],
    );

    // The pamtester version shipped by popular distributions (such as Fedora 44 and earlier)
    // has a spelling mistake in its output. This construct ensures that the test succeeds
    // even in the face of this circumstance
    let expected_stdout = [
        "pamtester: successfully opened a session\n",
        "pamtester: sucessfully opened a session\n",
    ];
    let expected_stderr = format!("username: {test_username}\n");
    let actual_stdout = String::from_utf8_lossy(&output.stdout);
    let actual_stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "stdout: {actual_stdout} stderr: {actual_stderr}"
    );
    assert!(
        expected_stdout.contains(&actual_stdout.as_ref()),
        "stdout: {actual_stdout} stderr: {actual_stderr}"
    );
    assert_eq!(expected_stderr, String::from_utf8_lossy(&output.stderr));
}
