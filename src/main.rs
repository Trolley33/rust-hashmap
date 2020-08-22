mod lib;
use lib::HashMap;

fn main() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("test", "value");
    // Attempt to retrieve value from hashmap, print if successful.
    if let Some(value) = map.get(&"tes") {
        println!("{}: {}", &"tes", value);
    } else {
        println!("{} has no value.", &"tes");
    }
}
