extern crate enum_set;
extern crate pcre;

use enum_set::{EnumSet};
use pcre::{CompileOption, Pcre, StudyOption};

#[test]
#[should_panic]
fn test_compile_nul() {
    // Nul bytes are not allowed in the pattern string.
    drop(Pcre::compile("\0abc"));
}

#[test]
fn test_compile_bad_pattern() {
    let err = Pcre::compile("[").unwrap_err();
    assert_eq!(err.offset(), 1);
}

#[test]
#[should_panic]
fn test_compile_bad_pattern2() {
    Pcre::compile("[").unwrap(); // Should be Err, will fail.
}

#[test]
fn test_compile_capture_count() {
    let re = Pcre::compile("(?:abc)(def)").unwrap();
    assert_eq!(re.capture_count(), 1);
}

#[test]
fn test_exec_basic() {
    let mut re = Pcre::compile("^...$").unwrap();
    assert_eq!(re.capture_count(), 0);
    let m = re.exec("abc").unwrap();
    assert_eq!(m.group(0), "abc");
}

#[test]
fn test_exec_no_match() {
    let mut re = Pcre::compile("abc").unwrap();
    assert!(re.exec("def").is_none());
}

#[test]
fn test_exec_nul_byte() {
    // Nul bytes *are* allowed in subject strings, however.
    let mut re = Pcre::compile("abc\\0def").unwrap();
    let m = re.exec("abc\0def").unwrap();
    assert_eq!(m.group(0), "abc\0def");
}

#[test]
fn test_exec_from_basic() {
    let mut re = Pcre::compile("abc").unwrap();
    let subject = "abcabc";
    let m1 = re.exec_from(subject, 1).unwrap();
    assert_eq!(m1.group_start(0), 3);
    assert_eq!(m1.group_end(0), 6);
    assert_eq!(m1.group_len(0), 3);
    let m2 = re.exec(subject).unwrap();
    assert_eq!(m2.group_start(0), 0);
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
fn test_matches_basic() {
    let subject = "\0abc1111abcabc___ababc+a";
    let mut re = Pcre::compile("abc").unwrap();
    let mut it = re.matches(subject);

    let mut opt_m = it.next();
    assert!(opt_m.is_some());
    let mut m = opt_m.unwrap();
    assert_eq!(m.group_start(0), 1);
    assert_eq!(m.group_end(0), 4);

    let opt_m2 = it.next();
    assert!(opt_m2.is_some());
    let m2 = opt_m2.unwrap();
    assert_eq!(m2.group_start(0), 8);
    assert_eq!(m2.group_end(0), 11);
    // Verify that getting the next match has not changed the first match data.
    assert_eq!(m.group_start(0), 1);
    assert_eq!(m.group_end(0), 4);

    opt_m = it.next();
    assert!(opt_m.is_some());
    m = opt_m.unwrap();
    assert_eq!(m.group_start(0), 11);
    assert_eq!(m.group_end(0), 14);

    opt_m = it.next();
    assert!(opt_m.is_some());
    m = opt_m.unwrap();
    assert_eq!(m.group_start(0), 19);
    assert_eq!(m.group_end(0), 22);

    opt_m = it.next();
    assert!(opt_m.is_none());
}

#[test]
fn test_extra_mark() {
    let pattern = "X(*MARK:A)Y|X(*MARK:B)Z";
    let subject1 = "XY";
    let subject2 = "XZ";

    let mut compile_options: EnumSet<CompileOption> = EnumSet::new();
    compile_options.insert(CompileOption::Extra);

    let mut re = Pcre::compile_with_options(pattern, &compile_options).unwrap();

    // first try to get the mark from the compile to make sure it fails
    assert_eq!(re.mark(), None);

    let mut study_options: EnumSet<StudyOption> = EnumSet::new();
    //study_options.add(StudyOption::StudyExtraNeeded);
    study_options.insert(StudyOption::StudyJitCompile);
    let study = re.study_with_options(&study_options);
    // Double check to make sure the study worked
    assert!(study);

    // Now after studying, we still should not be able to get the mark (since we still need 
    // to set the option in the extra AND execute it)
    assert_eq!(re.mark(), None);

    // set that I am using the extra mark field
    let extra = re.enable_mark();
    // This will fail only if I didn't study first
    assert!(extra);

    // We still haven't run the pcre_exec yet so get mark should be None still
    assert_eq!(re.mark(), None);

    // Now execute and we should be able to get the mark
    let opt_m1 = re.exec(subject1);
    assert!(opt_m1.is_some());

    // It should match XY 
    let m1 = opt_m1.unwrap();
    assert_eq!(m1.group(0), "XY");

    // and the marked value should be A
    let mark1 = re.mark();
    assert!(mark1.is_some());
    assert_eq!(mark1.unwrap(), "A");

    let opt_m2 = re.exec(subject2);
    assert!(opt_m2.is_some());

    let m2 = opt_m2.unwrap();
    // It should match XZ
    assert_eq!(m2.group(0), "XZ");

    // and the marked value should be B
    assert_eq!(re.mark().unwrap(), "B");
}
