use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug, Default)]
pub struct Arena<T> {
    items: Vec<T>,
}

impl<T> Arena<T> {
    pub fn push(&mut self, value: T) -> usize {
        let index = self.items.len();
        self.items.push(value);
        index
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Interner<T>
where
    T: Clone + Eq + Hash,
{
    items: Vec<T>,
    index_by_value: HashMap<T, usize>,
}

impl<T> Interner<T>
where
    T: Clone + Eq + Hash,
{
    pub fn intern(&mut self, value: T) -> usize {
        if let Some(index) = self.index_by_value.get(&value) {
            return *index;
        }

        let index = self.items.len();
        self.items.push(value.clone());
        self.index_by_value.insert(value, index);
        index
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
}
