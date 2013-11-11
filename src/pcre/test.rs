extern mod pcre;

use pcre::Pcre;

#[test]
#[should_fail]
fn test_compile_nul() {
    // Nul bytes are not allowed in the pattern string.
    Pcre::compile("\0abc");
}

#[test]
fn test_compile_bad_pattern() {
    let err = Pcre::compile("[").unwrap_err();
    assert_eq!(err.offset(), 1u);
}

#[test]
#[should_fail]
fn test_compile_bad_pattern2() {
    Pcre::compile("[").unwrap(); // Should be Err, will fail.
}

#[test]
fn test_compile_capture_count() {
    let re = Pcre::compile("(?:abc)(def)").unwrap();
    assert_eq!(re.capture_count(), 1u);
}

#[test]
fn test_exec_basic() {
    let re = Pcre::compile("^...$").unwrap();
    assert_eq!(re.capture_count(), 0u);
    let m = re.exec("abc").unwrap();
    assert_eq!(m.group(0), "abc");
}

#[test]
fn test_exec_no_match() {
    let re = Pcre::compile("abc").unwrap();
    assert!(re.exec("def").is_none());
}

#[test]
fn test_exec_nul_byte() {
    // Nul bytes *are* allowed in subject strings, however.
    let re = Pcre::compile("abc\\0def").unwrap();
    let m = re.exec("abc\0def").unwrap();
    assert_eq!(m.group(0), "abc\0def");
}

#[test]
fn test_exec_from_basic() {
    let re = Pcre::compile("abc").unwrap();
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
    let mut re = Pcre::compile("abc").unwrap();
    let mut study_res = re.study();
    assert!(study_res);
    // Re-study the pattern two more times (to check for leaks when the test program
    // is run through Valgrind).
    study_res = re.study();
    assert!(study_res);
    study_res = re.study();
    assert!(study_res);
}

#[test]
fn test_match_iter_basic() {
    let subject = "\0abc1111abcabc___ababc+a";
    let mut it = {
        let re = Pcre::compile("abc").unwrap();
        re.match_iter(subject)

        // The MatchIterator should retain a reference to the `pcre`.
    };

    let mut opt_m = it.next();
    assert!(opt_m.is_some());
    let mut m = opt_m.unwrap();
    assert_eq!(m.group_start(0u), 1u);
    assert_eq!(m.group_end(0u), 4u);

    let opt_m2 = it.next();
    assert!(opt_m2.is_some());
    let m2 = opt_m2.unwrap();
    assert_eq!(m2.group_start(0u), 8u);
    assert_eq!(m2.group_end(0u), 11u);
    // Verify that getting the next match has not changed the first match data.
    assert_eq!(m.group_start(0u), 1u);
    assert_eq!(m.group_end(0u), 4u);

    opt_m = it.next();
    assert!(opt_m.is_some());
    m = opt_m.unwrap();
    assert_eq!(m.group_start(0u), 11u);
    assert_eq!(m.group_end(0u), 14u);

    opt_m = it.next();
    assert!(opt_m.is_some());
    m = opt_m.unwrap();
    assert_eq!(m.group_start(0u), 19u);
    assert_eq!(m.group_end(0u), 22u);

    opt_m = it.next();
    assert!(opt_m.is_none());
}
