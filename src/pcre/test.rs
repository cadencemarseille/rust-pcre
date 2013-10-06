extern mod pcre;

use pcre::Pcre;

#[test]
#[should_fail]
fn test_compile_nul() {
    // Nul bytes are not allowed in the pattern string.
    Pcre::compile("\0abc");
}

#[test]
#[should_fail]
fn test_compile_bad_pattern() {
    Pcre::compile("[");
}

#[test]
fn test_compile_capture_count() {
    let re = Pcre::compile("(?:abc)(def)");
    assert_eq!(re.capture_count(), 1u);
}

#[test]
fn test_exec_basic() {
    let re = Pcre::compile("^...$");
    assert_eq!(re.capture_count(), 0u);
    let m = re.exec("abc").unwrap();
    assert_eq!(m.group(0), "abc");
}

#[test]
fn test_exec_no_match() {
    let re = Pcre::compile("abc");
    assert!(re.exec("def").is_none());
}

#[test]
fn test_exec_nul_byte() {
    // Nul bytes *are* allowed in subject strings, however.
    let re = Pcre::compile("abc\\0def");
    let m = re.exec("abc\0def").unwrap();
    assert_eq!(m.group(0), "abc\0def");
}

#[test]
fn test_exec_from_basic() {
    let re = Pcre::compile("abc");
    let subject = "abcabc";
    let m1 = re.exec_from(subject, 1u).unwrap();
    assert_eq!(m1.group_start(0u), 3u);
    assert_eq!(m1.group_end(0u), 6u);
    assert_eq!(m1.group_len(0u), 3u);
    let m2 = re.exec(subject).unwrap();
    assert_eq!(m2.group_start(0u), 0u);
}

#[test]
fn test_study_basic() {
    let mut re = Pcre::compile("abc");
    let mut study_res = re.study();
    assert!(study_res);
    // Re-study the pattern two more times (to check for leaks when the test program
    // is run through Valgrind).
    study_res = re.study();
    assert!(study_res);
    study_res = re.study();
    assert!(study_res);
}
