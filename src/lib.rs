use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

type Pair<K, V> = (K, V);
#[derive(Debug)]
pub struct HashMap<K: std::hash::Hash + Eq, V> {
    buckets: [Vec<Pair<K, V>>; 16],
}

impl<K, V> HashMap<K, V>
where
    K: std::hash::Hash + Eq + std::fmt::Debug,
    V: std::fmt::Debug + Clone,
{
    /// Initialise a new hashmap, by default we specificy 16 empty buckets.
    pub fn new() -> Self {
        let mut buckets: [Vec<Pair<K, V>>; 16] = Default::default();
        for i in 0..buckets.len() {
            buckets[i] = vec![];
        }

        Self { buckets }
    }

    /// Inserts given (key, value) pair into hashmap.
    /// Returns value of previous element if one already exists.
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: std::fmt::Debug,
    {
        let bucket_index = self.get_bucket_index(&key);

        for item_index in 0..self.buckets[bucket_index].len() {
            if self.buckets[bucket_index][item_index].0 == key {
                // * Remove current pairing, and return the values (move ownership to this function)
                // * - Note we ignore key since we already have a copy from params).
                let (_, old_value) = self.buckets[bucket_index].remove(item_index);

                // Re-insert new pairing.
                self.buckets[bucket_index].push((key, value));

                // Return the old value.
                return Some(old_value);
            }
        }
        // * If we don't find anything from the loop (no return) - add element to bucket.
        self.buckets[bucket_index].push((key, value));
        None
    }

    /// Given reference to key, return a reference to the value if it exists.
    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket_index = self.get_bucket_index(key);

        if self.buckets[bucket_index].len() > 0 {
            for pair in self.buckets[bucket_index].iter() {
                if pair.0 == *key {
                    return Some(&pair.1);
                }
            }
        }

        None
    }

    /// Given a reference by key, if key is in hashmap remove its pairing and return value.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let bucket_index = self.get_bucket_index(key);

        for item_index in 0..self.buckets[bucket_index].len() {
            if self.buckets[bucket_index][item_index].0 == *key {
                let (_, old_value) = self.buckets[bucket_index].remove(item_index);
                return Some(old_value);
            }
        }
        None
    }

    /// Internal helper function, returns bucket index based on hash of key.
    fn get_bucket_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        hasher.finish() as usize % self.buckets.len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        // create a new HashMap
        let mut map = HashMap::new();

        // insert key/value pairs into the HashMap
        assert_eq!(map.insert("foo", "bar"), None);
        // if an item already exists for that value, it should return the old value
        assert_eq!(map.insert("foo", "lol"), Some("bar"));

        // get a value based on its key
        assert_eq!(map.get(&"foo"), Some(&"lol"));
        // you should be able to do this multiple times
        assert_eq!(map.get(&"foo"), Some(&"lol"));
        // if no value exists for a key, return None
        assert_eq!(map.get(&"qux"), None);

        // remove a value for a key
        assert_eq!(map.remove(&"foo"), Some("lol"));
        // once it no longer exists in the map, it should return None
        assert_eq!(map.get(&"foo"), None);
    }
}
