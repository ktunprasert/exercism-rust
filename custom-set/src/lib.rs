use std::fmt::Debug;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, PartialEq, Default)]
pub struct CustomSet<T: Eq + Hash + Clone + Default + Debug> {
    map: HashMap<T, ()>,
}

impl<T: Eq + Hash + Clone + Default + Debug> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        Self {
            map: input.iter().fold(HashMap::default(), |mut set, k| {
                set.insert(k.to_owned(), ());
                set
            }),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.map.contains_key(element)
    }

    pub fn add(&mut self, el: T) {
        self.map.insert(el, ());
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.map.keys().all(|k| other.contains(k))
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.map.keys().all(|k| !other.contains(k))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        self.map
            .keys()
            .filter(|k| other.contains(k))
            .fold(Self::default(), |mut set, k| {
                set.add(k.to_owned());
                set
            })
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        self.map
            .keys()
            .filter(|k| !other.contains(k))
            .fold(Self::default(), |mut set, k| {
                set.add(k.to_owned());
                set
            })
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        self.map
            .keys()
            .chain(other.map.keys())
            .filter(|k| self.contains(k) || other.contains(k))
            .inspect(|s| println!("{:?}", s))
            .fold(Self::default(), |mut set, k| {
                set.add(k.to_owned());
                set
            })
    }
}
