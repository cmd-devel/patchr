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
}