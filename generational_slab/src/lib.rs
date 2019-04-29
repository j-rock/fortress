use std::{
    fmt,
    iter::IntoIterator,
};

#[derive(Clone)]
pub struct Slab<T> {
    entries: Vec<Entry<T>>,
    generation: u64,
    len: usize,
    next: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Key {
    index: usize,
    generation: u64,
}

impl Key {
    pub fn to_raw(self) -> usize {
        self.index
    }
}

#[derive(Debug)]
pub struct VacantEntry<'a, T: 'a> {
    slab: &'a mut Slab<T>,
    key: Key,
}

pub struct IntoIter<T> {
    entries: std::vec::IntoIter<Entry<T>>,
    curr: usize,
}

pub struct Iter<'a, T: 'a> {
    entries: std::slice::Iter<'a, Entry<T>>,
    curr: usize,
}

pub struct IterMut<'a, T: 'a> {
    entries: std::slice::IterMut<'a, Entry<T>>,
    curr: usize,
}

#[derive(Clone)]
enum Entry<T> {
    Vacant(usize),
    Occupied(T, u64),
}

impl<T> Slab<T> {
    pub fn new() -> Slab<T> {
        Slab::with_capacity(10)
    }

    pub fn with_capacity(capacity: usize) -> Slab<T> {
        Slab {
            entries: Vec::with_capacity(capacity),
            generation: 0,
            next: 0,
            len: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.entries.capacity()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.generation = 0;
        self.len = 0;
        self.next = 0;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            entries: self.entries.iter(),
            curr: 0,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            entries: self.entries.iter_mut(),
            curr: 0,
        }
    }

    pub fn get(&self, key: Key) -> Option<&T> {
        match self.entries.get(key.index) {
            Some(&Entry::Occupied(ref val, generation)) if generation == key.generation => Some(val),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, key: Key) -> Option<&mut T> {
        match self.entries.get_mut(key.index) {
            Some(&mut Entry::Occupied(ref mut val, generation)) if generation == key.generation => Some(val),
            _ => None,
        }
    }

    pub fn insert(&mut self, val: T) -> Option<Key> {
        let key = Key {
            index: self.next,
            generation: self.generation,
        };

        if self.insert_at(key, val) {
            Some(key)
        } else {
            None
        }
    }

    pub fn vacant_entry(&mut self) -> VacantEntry<T> {
        VacantEntry {
            key: Key {
                index: self.next,
                generation: self.generation,
            },
            slab: self,
        }
    }

    fn insert_at(&mut self, key: Key, val: T) -> bool {
        self.len += 1;

        if key.index == self.entries.len() {
            self.entries.push(Entry::Occupied(val, key.generation));
            self.next = key.index + 1;
        } else {
            if let Some(&Entry::Vacant(next)) = self.entries.get(key.index) {
                self.next = next;
            }
            self.entries[key.index] = Entry::Occupied(val, key.generation);
        }

        true
    }

    pub fn remove(&mut self, key: Key) -> Option<T> {
        let prev = std::mem::replace(&mut self.entries[key.index], Entry::Vacant(self.next));

        match prev {
            Entry::Vacant(next) => {
                self.entries[key.index] = Entry::Vacant(next);
                None
            },
            Entry::Occupied(val, gen) => {
                if gen == key.generation {
                    self.len -= 1;
                    self.next = key.index;
                    Some(val)
                } else {
                    self.entries[key.index] = Entry::Occupied(val, gen);
                    None
                }
            },
        }
    }

    pub fn contains(&self, key: Key) -> bool {
        match self.entries.get(key.index) {
            Some(&Entry::Occupied(_, generation)) => {
                generation == key.generation
            },
            _ => false,
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        if self.capacity() - self.len >= additional {
            return;
        }
        let need_add = self.len + additional - self.entries.len();
        self.entries.reserve(need_add);
    }
}

impl<T> IntoIterator for Slab<T> {
    type Item = (usize, T);
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            entries: self.entries.into_iter(),
            curr: 0,
        }
    }
}

impl<'a, T> IntoIterator for &'a Slab<T> {
    type Item = (usize, &'a T);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Slab<T> {
    type Item = (usize, &'a mut T);
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

impl<T> fmt::Debug for Slab<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Slab {{ len: {}, cap: {} }}",
            self.len,
            self.capacity()
        )
    }
}

impl<'a, T> VacantEntry<'a, T> {
    pub fn insert(self, val: T) -> bool {
        self.slab.insert_at(self.key, val)
    }

    pub fn key(&self) -> Key {
        self.key
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = (usize, T);

    fn next(&mut self) -> Option<(usize, T)> {
        while let Some(entry) = self.entries.next() {
            let curr = self.curr;
            self.curr += 1;

            if let Entry::Occupied(v, _) = entry {
                return Some((curr, v));
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.entries.len()))
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<(usize, &'a T)> {
        while let Some(entry) = self.entries.next() {
            let curr = self.curr;
            self.curr += 1;

            if let Entry::Occupied(ref v, _) = *entry {
                return Some((curr, v));
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.entries.len()))
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (usize, &'a mut T);

    fn next(&mut self) -> Option<(usize, &'a mut T)> {
        while let Some(entry) = self.entries.next() {
            let curr = self.curr;
            self.curr += 1;

            if let Entry::Occupied(ref mut v, _) = *entry {
                return Some((curr, v));
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.entries.len()))
    }
}

