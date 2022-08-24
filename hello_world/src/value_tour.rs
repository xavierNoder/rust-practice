use std::collections::HashMap;

pub fn value_tour() -> HashMap<String, String> {
    let authenticate = true;
    let mut total = 0usize;
    total += 1;
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("hello".into(),"value".into());
    return map;
}