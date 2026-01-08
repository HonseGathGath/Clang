// 1. Finish the function
pub fn unique_items<T>(collection: T) -> Vec<String>
where
    T: IntoIterator,
    T::Item: AsRef<str>,
{
    let mut v: Vec<String> = collection
        .into_iter()
        .map(|item| item.as_ref().trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    v.sort();

    v.dedup();

    v
}

pub fn main() {
    let product_ids = vec![
        "abc123".to_string(),
        "  ".to_string(),
        "def456".to_string(),
        "abc123".to_string(),
        "ghi789".to_string(),
        "ghi789".to_string(),
        "   def456".to_string(),
    ];

    let unique_ids = unique_items(product_ids.into_iter());
    assert_eq!(unique_ids, vec!["abc123", "def456", "ghi789"]);
}
