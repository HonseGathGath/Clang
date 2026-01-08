use std::{collections::HashMap, hash::Hash};

// 1. Finish the trait definition
pub trait KeyValueStore {
    type Key;
    type Value;
    fn set(&mut self, key: Self::Key, value: Self::Value);
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
}

// 2. Implement the trait for InMemoryStore
// Make sure the fields are public
pub struct InMemoryStore<K, V> {
    pub storage: HashMap<K, V>,
}

// 3. Implement the trait for InMemoryStore

impl<K, V> KeyValueStore for InMemoryStore<K, V>
where
    K: Eq + Hash,
{
    type Key = K;
    type Value = V;
    fn set(&mut self, key: Self::Key, value: Self::Value) {
        self.storage.insert(key, value);
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        self.storage.get(&key)
    }
}

// Example usage
pub fn main() {
    let mut store: InMemoryStore<String, String> = InMemoryStore {
        storage: HashMap::new(),
    };

    store.set("name".to_string(), "Rust".to_string());
    assert_eq!(store.get(&"name".to_string()), Some(&"Rust".to_string()));

    store.set("language".to_string(), "Rust".to_string());
    assert_eq!(
        store.get(&"language".to_string()),
        Some(&"Rust".to_string())
    );

    assert_eq!(store.get(&"non_existent".to_string()), None);
}
