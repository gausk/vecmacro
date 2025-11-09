#[macro_export]
macro_rules! hashmap {
    ( $($key:expr => $value:expr ),* $(,)?) => {
        std::collections::HashMap::<_, _>::from([ $( ($key, $value) ),* ])
    }
}

#[test]
fn test_hashmap() {
    let map = hashmap! {
       "foo" => 1,
       "bar" => 2,
       "baz" => 3,
       "qux" => 4,
    };
    assert_eq!(map.get("baz"), Some(&3));
    assert_eq!(map["bar"], 2);
}
