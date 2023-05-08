use super::*;

#[test]
fn test_multiple_strip_diff_to_files() {
    let multi_file_arg = String::from(
        ".gitlab-ci.yml                            |  2 +-
CHANGELOG.md                              | 35 ++++++++++++++++++++++++++++++++-
path/to/release.yaml      |  2 +-
another/path/to/release.yaml | 25 +++++++++++++++++++----
4 files changed, 58 insertions(+), 6 deletions(-) ",
    );

    let multi_file_want = String::from(
        ".gitlab-ci.yml
CHANGELOG.md
path/to/release.yaml
another/path/to/release.yaml",
    );

    assert_eq!(strip_diff_to_files(multi_file_arg), multi_file_want);
}

#[test]
fn test_single_strip_diff_to_files() {
    let single_file_arg = String::from(
        "CHANGELOG.md | 7 +++++++
1 file changed, 7 insertions(+)",
    );
    let single_file_want = String::from("CHANGELOG.md");
    assert_eq!(strip_diff_to_files(single_file_arg), single_file_want);
}

#[test]
fn test_zero_strip_diff_to_files() {
    let zero_file_arg = String::from("0 files changed, 0 insertions(+), 0 deletions(-)");
    let zero_file_want = String::from("");
    assert_eq!(strip_diff_to_files(zero_file_arg), zero_file_want);
}

#[test]
fn test_invalid_strip_diff_to_files() {
    let zero_file_arg = String::from(
        "some invalid
multi line string",
    );
    let zero_file_want = String::from("");
    assert_eq!(strip_diff_to_files(zero_file_arg), zero_file_want);
}
