use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();

    map.insert("foo".to_string(), "bar".to_string());
    map.insert("hello".to_string(), "world".to_string());

    // Serialize
    let yaml = serde_yaml::to_string(&map)
        .expect("unable to serialize structure");
    println!("{}", yaml);

    // Deserialize
    let deserialized_map: BTreeMap<String, String> = serde_yaml::from_str(&yaml)
        .expect("unable to deserialize structure");
    assert_eq!(map, deserialized_map);
    println!("deserialization successful");
}
