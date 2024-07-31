use crate::series::Series;

#[test]
fn test_series_creation() {
    // Invalid
    assert!(Series::new("", "The title").is_none());
    assert!(Series::new("TheName", "").is_none());
    assert!(Series::new("The name", "The title").is_none());
    assert!(Series::new("Thename$", "The title").is_none());
    assert!(Series::new("Thename$", "The title").is_none());
    
    // Valid
    assert!(Series::new("Thename", "The title").is_some());
    assert!(Series::new("The_name", "The title").is_some());
    assert!(Series::new("The_name", "My-long_and_weird-title").is_some());
}

#[test]
fn test_series_revs() {
    let mut series = Series::new("Name", "Title").unwrap();
    assert_eq!(series.current_revision(), 1);
    for i in 2..10 {
        series.add_revision();
        assert_eq!(series.current_revision(), i);
    }
    
    assert_eq!(series.current_revision(), 9);
    series.delete_revision(0);
    assert_eq!(series.current_revision(), 9);
    series.delete_revision(1);
    assert_eq!(series.current_revision(), 9);


    for i in (2..10).rev() {
        assert_eq!(series.current_revision(), i);
        series.delete_revision(2);
    }
    assert_eq!(series.current_revision(), 1);
    series.add_revision();
    series.add_revision();
    series.add_revision();
    series.add_revision();
    assert_eq!(series.current_revision(), 5);
    series.delete_revision(0);
    assert_eq!(series.current_revision(), 5);
    series.delete_revision(1);
    assert_eq!(series.current_revision(), 5);
}

#[test]
fn test_series_revs_data() {
    let gen_content = |i| { format!("This is revision {}", i) };
    let mut series = Series::new("Name", "Title").unwrap();
    for i in 2..10 {
        series.add_revision();
        let rev = series.revision_mut(i).unwrap();
        assert_eq!(rev.content(), "");
        rev.set_content(gen_content(i).as_str());
    }

    assert_eq!(series.revision_mut(6).unwrap().content(), gen_content(6).as_str());
    series.delete_revision(6);
    assert_eq!(series.revision_mut(8).unwrap().content(), gen_content(9).as_str());
    series.delete_revision(8);
    assert_eq!(series.revision_mut(2).unwrap().content(), gen_content(2).as_str());
    series.delete_revision(2);
    assert_eq!(series.revision_mut(5).unwrap().content(), gen_content(7).as_str());
    series.delete_revision(5);
    assert_eq!(series.revision_mut(5).unwrap().content(), gen_content(8).as_str());
    series.delete_revision(5);
    assert_eq!(series.revision_mut(3).unwrap().content(), gen_content(4).as_str());
    series.delete_revision(3);
    assert_eq!(series.revision_mut(2).unwrap().content(), gen_content(3).as_str());
    series.delete_revision(2);
    assert_eq!(series.revision_mut(2).unwrap().content(), gen_content(5).as_str());
    series.delete_revision(2);
    assert_eq!(series.current_revision(), 1);
    series.delete_revision(1);
    assert_eq!(series.current_revision(), 1);
    series.delete_revision(0);
    assert_eq!(series.current_revision(), 1);
}

#[test]
fn test_series_name() {
    let mut series = Series::new("name", "title").unwrap();

    assert!(series.set_name("").is_err());
    assert_eq!(series.name(), "name");
    assert!(series.set_name("test$a").is_err());
    assert!(series.set_name("test-a").is_ok());
    assert_eq!(series.name(), "test-a");
    assert!(series.set_name("a name").is_err());
    assert!(series.set_name("a_-_name").is_ok());
    assert!(series.set_title("aAgGdDpP---RTY").is_ok());
    // TODO: add more tests
}

#[test]
fn test_series_short_name() {
    let mut series = Series::new("name", "title").unwrap();

    assert_eq!(series.short_name(), "");
    assert!(series.set_short_name("a").is_ok());
    assert_eq!(series.short_name(), "a");

    assert!(series.set_short_name("a space").is_err());

    assert!(series.set_short_name("not-a-space").is_ok());
    assert_eq!(series.short_name(), "not-a-space");

    assert!(series.set_short_name("not_a_space").is_ok());
    assert_eq!(series.short_name(), "not_a_space");

    assert!(series.set_short_name("not+allowed").is_err());
    assert!(series.set_short_name("fancy-name-that-is-not-so-short").is_err());

    assert!(series.set_short_name("").is_ok());
    assert_eq!(series.short_name(), "");
    // TODO: add more test
}

#[test]
fn test_series_title() {
    let mut series = Series::new("name", "title").unwrap();

    assert!(series.set_title("").is_err());
    assert_eq!(series.title(), "title");
    assert!(series.set_title("test$a").is_err());
    assert!(series.set_title("test-a").is_ok());
    assert_eq!(series.title(), "test-a");
    assert!(series.set_title("a title").is_ok());
    assert!(series.set_title("a_-title").is_ok());
    assert!(series.set_title("aAgGdDpP---RTY").is_ok());
    // TODO: add more tests
}

#[test]
fn test_series_cover_letter() {
    let mut series = Series::new("name", "title").unwrap();

    assert!(series.set_cover_letter("").is_ok());
    assert_eq!(series.cover_letter(), "");
    let cv = "test\n\n\t\na multi line \ncover\nletter";
    assert!(series.set_cover_letter(cv).is_ok());
    assert_eq!(series.cover_letter(), cv);

    let cv = "a".repeat(1500);
    assert!(series.set_cover_letter(cv.as_str()).is_ok());
    assert_eq!(series.cover_letter(), cv);

    assert!(series.set_cover_letter("").is_ok());
    assert_eq!(series.cover_letter(), "");
}