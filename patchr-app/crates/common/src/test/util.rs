use crate::util::{git::commit_hash_valid, input::sanitize_cc_list};

#[test]
fn test_sanitize_cc_list() {
    // valid
    assert!(sanitize_cc_list("my@email.com").unwrap() == "my@email.com");
    assert!(sanitize_cc_list(" \t   my@email.com    \t   \t").unwrap() == "my@email.com");
    assert!(sanitize_cc_list("\t    my@email.com,my.second@email.com     ").unwrap() == "my@email.com,my.second@email.com");
    assert!(sanitize_cc_list("\t    a@a.com,b@b.com,c@c.com,d@d.com     ").unwrap() == "a@a.com,b@b.com,c@c.com,d@d.com");

    // invalid
    assert!(sanitize_cc_list("").is_none());
    assert!(sanitize_cc_list("     ").is_none());
    assert!(sanitize_cc_list("  ,,  ,,,, ,,  ").is_none());
    assert!(sanitize_cc_list("myemail.com").is_none());
    assert!(sanitize_cc_list("a@a.com,").is_none()); // trailing comma
    assert!(sanitize_cc_list(",a@a.com").is_none()); // leading comma
    assert!(sanitize_cc_list("a@a.com,,a@a.com").is_none()); // empty field
}

#[test]
fn test_commit_hash_validation() {
    // valid
    assert!(commit_hash_valid("0000000000000000000000000000000000000000"));
    assert!(commit_hash_valid("1111111111111111111111111111111111111111"));
    assert!(commit_hash_valid("2222222222222222222222222222222222222222"));
    assert!(commit_hash_valid("3333333333333333333333333333333333333333"));
    assert!(commit_hash_valid("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
    assert!(commit_hash_valid("ffffffffffffffffffffffffffffffffffffffff"));
    assert!(commit_hash_valid("7ac63f28fbb52736232655da7c817c181a91f384"));
    assert!(commit_hash_valid("a783a72463f175ca56b5d31be15f3ca527636804"));
    
    // invalid
    assert!(!commit_hash_valid(""));
    assert!(!commit_hash_valid("fffffffffffffffff ffffffffffffffffffffff"));
    assert!(!commit_hash_valid("00000000000000000g0000000000000000000000"));
    assert!(!commit_hash_valid(" 000000000000000000000000000000000000000"));
    assert!(!commit_hash_valid("00000000000000000000000000000000000000"));
    assert!(!commit_hash_valid("000000000000000000000000000000000000000"));
}