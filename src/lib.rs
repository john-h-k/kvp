//! # kvp
//!
//! This crate is designed to provide a type, `KeyValuePair<TKey, TValue>`,
//! which can be used when someone wishes to "tag" a value with additional data,
//! while keeping its functional traits (`PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`) entirely on the key,
//! disregarding the tagged data. This can be used, for example, to have a [`BinaryHeap`] which acts more similarly
//! to a [`HashMap`] in its ability to store key-value data.
//!
//! ```
//! let heap = BinaryHeap::new();
//!
//! heap.push(KeyValuePair { key: 50, value: "Hey, here's an associated value" })
//! // You can also use ::new syntax
//! heap.push(KeyValuePair::new(0, "another bit of associated data!"));
//! heap.push(KeyValuePair::new(22, "voila"));
//!
//! assert_matches!(heap.pop(), Some(50));
//! assert_matches!(heap.pop(), Some(22));
//! assert_matches!(heap.pop(), Some(0));
//! ```
//!
//! [`BinaryHeap`]: std::collections::BinaryHeap
//! [`HashMap`]: std::collections::HashMap

use core::{fmt, hash::Hash};

pub struct KeyValuePair<TKey, TValue> {
    pub key: TKey,
    pub value: TValue,
}

impl<TKey, TValue> KeyValuePair<TKey, TValue> {
    pub fn new(key: TKey, value: TValue) -> Self {
        Self { key, value }
    }
}

impl<TKey: PartialEq, TValue> PartialEq for KeyValuePair<TKey, TValue> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<TKey: Eq, TValue> Eq for KeyValuePair<TKey, TValue> {}

impl<TKey: PartialOrd, TValue> PartialOrd for KeyValuePair<TKey, TValue> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<TKey: Ord, TValue> Ord for KeyValuePair<TKey, TValue> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

impl<TKey: Hash, TValue> Hash for KeyValuePair<TKey, TValue> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state)
    }
}

impl<TKey: fmt::Debug, TValue: fmt::Debug> fmt::Debug for KeyValuePair<TKey, TValue> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Info")
            .field("key", &self.key)
            .field("value", &self.value)
            .finish()
    }
}

impl<TKey: Clone, TValue: Clone> Clone for KeyValuePair<TKey, TValue> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }
}

impl<TKey: Copy, TValue: Copy> Copy for KeyValuePair<TKey, TValue> {}

#[cfg(test)]
mod tests {
    use crate::KeyValuePair;

    struct NonOrderedType;

    #[test]
    fn eq_is_respected() {
        let mut a = vec![
            KeyValuePair {
                key: 2,
                value: NonOrderedType,
            },
            KeyValuePair {
                key: 1,
                value: NonOrderedType,
            },
            KeyValuePair {
                key: 0,
                value: NonOrderedType,
            },
            KeyValuePair {
                key: 6,
                value: NonOrderedType,
            },
        ];

        a.sort();

        assert_eq!(
            a.iter().map(|kv| kv.key).collect::<Vec<_>>(),
            vec![0, 1, 2, 6]
        );
    }
}
