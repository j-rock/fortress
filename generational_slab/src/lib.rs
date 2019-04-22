use std::iter::{FromIterator, IntoIterator};
use std::ops;
use std::vec;
use std::{fmt, mem};

#[derive(Clone)]
pub struct Slab<T> {
    entries: Vec<Entry<T>>,
    len: usize,
    next: usize,
}

impl<T> Default for Slab<T> {
    fn default() -> Self {
        Slab::new()
    }
}

#[derive(Debug)]
pub struct VacantEntry<'a, T: 'a> {
    slab: &'a mut Slab<T>,
    key: usize,
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

pub struct Drain<'a, T: 'a>(vec::Drain<'a, Entry<T>>);

#[derive(Clone)]
enum Entry<T> {
    Vacant(usize),
    Occupied(T),
}

impl<T> Slab<T> {
    pub fn new() -> Slab<T> {
        Slab::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Slab<T> {
        Slab {
            entries: Vec::with_capacity(capacity),
            next: 0,
            len: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.entries.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        if self.capacity() - self.len >= additional {
            return;
        }
        let need_add = additional - (self.entries.len() - self.len);
        self.entries.reserve(need_add);
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        if self.capacity() - self.len >= additional {
            return;
        }
        let need_add = additional - (self.entries.len() - self.len);
        self.entries.reserve_exact(need_add);
    }

    pub fn shrink_to_fit(&mut self) {
        let len_before = self.entries.len();
        while let Some(&Entry::Vacant(_)) = self.entries.last() {
            self.entries.pop();
        }

        if self.entries.len() != len_before {
            self.recreate_vacant_list();
        }

        self.entries.shrink_to_fit();
    }

    fn recreate_vacant_list(&mut self) {
        self.next = self.entries.len();
        let mut remaining_vacant = self.entries.len() - self.len;
        for (i, entry) in self.entries.iter_mut().enumerate().rev() {
            if remaining_vacant == 0 {
                break;
            }
            if let Entry::Vacant(ref mut next) = *entry {
                *next = self.next;
                self.next = i;
                remaining_vacant -= 1;
            }
        }
    }

    pub fn compact<F>(&mut self, mut rekey: F)
    where
        F: FnMut(&mut T, usize, usize) -> bool,
    {
        struct CleanupGuard<'a, T: 'a> {
            slab: &'a mut Slab<T>,
            decrement: bool,
        }
        impl<'a, T: 'a> Drop for CleanupGuard<'a, T> {
            fn drop(&mut self) {
                if self.decrement {
                    self.slab.len -= 1;
                }
                self.slab.recreate_vacant_list();
            }
        }
        let mut guard = CleanupGuard {
            slab: self,
            decrement: true,
        };

        let mut occupied_until = 0;
        while guard.slab.entries.len() > guard.slab.len {
            if let Some(Entry::Occupied(mut value)) = guard.slab.entries.pop() {
                while let Some(&Entry::Occupied(_)) = guard.slab.entries.get(occupied_until) {
                    occupied_until += 1;
                }
                if !rekey(&mut value, guard.slab.entries.len(), occupied_until) {
                    guard.slab.entries.push(Entry::Occupied(value));
                    guard.decrement = false;
                    guard.slab.entries.shrink_to_fit();
                    return;
                }
                guard.slab.entries[occupied_until] = Entry::Occupied(value);
                occupied_until += 1;
            }
        }
        guard.slab.next = guard.slab.len;
        guard.slab.entries.shrink_to_fit();
        mem::forget(guard);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
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

    pub fn get(&self, key: usize) -> Option<&T> {
        match self.entries.get(key) {
            Some(&Entry::Occupied(ref val)) => Some(val),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        match self.entries.get_mut(key) {
            Some(&mut Entry::Occupied(ref mut val)) => Some(val),
            _ => None,
        }
    }

    pub unsafe fn get_unchecked(&self, key: usize) -> &T {
        match *self.entries.get_unchecked(key) {
            Entry::Occupied(ref val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn get_unchecked_mut(&mut self, key: usize) -> &mut T {
        match *self.entries.get_unchecked_mut(key) {
            Entry::Occupied(ref mut val) => val,
            _ => unreachable!(),
        }
    }

    pub fn key_of(&self, present_element: &T) -> usize {
        let element_ptr = present_element as *const T as usize;
        let base_ptr = self.entries.as_ptr() as usize;
        // Use wrapping subtraction in case the reference is bad
        let byte_offset = element_ptr.wrapping_sub(base_ptr);
        // The division rounds away any offset of T inside Entry
        // The size of Entry<T> is never zero even if T is due to Vacant(usize)
        let key = byte_offset / mem::size_of::<Entry<T>>();
        // Prevent returning unspecified (but out of bounds) values
        if key >= self.entries.len() {
            panic!("The reference points to a value outside this slab");
        }
        // The reference cannot point to a vacant entry, because then it would not be valid
        key
    }

    pub fn insert(&mut self, val: T) -> usize {
        let key = self.next;

        self.insert_at(key, val);

        key
    }

    pub fn vacant_entry(&mut self) -> VacantEntry<T> {
        VacantEntry {
            key: self.next,
            slab: self,
        }
    }

    fn insert_at(&mut self, key: usize, val: T) {
        self.len += 1;

        if key == self.entries.len() {
            self.entries.push(Entry::Occupied(val));
            self.next = key + 1;
        } else {
            self.next = match self.entries.get(key) {
                Some(&Entry::Vacant(next)) => next,
                _ => unreachable!(),
            };
            self.entries[key] = Entry::Occupied(val);
        }
    }

    pub fn remove(&mut self, key: usize) -> T {
        if let Some(entry) = self.entries.get_mut(key) {
            // Swap the entry at the provided value
            let prev = mem::replace(entry, Entry::Vacant(self.next));

            match prev {
                Entry::Occupied(val) => {
                    self.len -= 1;
                    self.next = key;
                    return val;
                }
                _ => {
                    // Woops, the entry is actually vacant, restore the state
                    *entry = prev;
                }
            }
        }
        panic!("invalid key");
    }

    pub fn contains(&self, key: usize) -> bool {
        match self.entries.get(key) {
            Some(&Entry::Occupied(_)) => true,
            _ => false,
        }
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, &mut T) -> bool,
    {
        for i in 0..self.entries.len() {
            let keep = match self.entries[i] {
                Entry::Occupied(ref mut v) => f(i, v),
                _ => true,
            };

            if !keep {
                self.remove(i);
            }
        }
    }

    pub fn drain(&mut self) -> Drain<T> {
        self.len = 0;
        self.next = 0;
        Drain(self.entries.drain(..))
    }
}

impl<T> ops::Index<usize> for Slab<T> {
    type Output = T;

    fn index(&self, key: usize) -> &T {
        match self.entries.get(key) {
            Some(&Entry::Occupied(ref v)) => v,
            _ => panic!("invalid key"),
        }
    }
}

impl<T> ops::IndexMut<usize> for Slab<T> {
    fn index_mut(&mut self, key: usize) -> &mut T {
        match self.entries.get_mut(key) {
            Some(&mut Entry::Occupied(ref mut v)) => v,
            _ => panic!("invalid key"),
        }
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

impl<T> FromIterator<(usize, T)> for Slab<T> {
    fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = (usize, T)>,
    {
        let iterator = iterable.into_iter();
        let mut slab = Self::with_capacity(iterator.size_hint().0);

        let mut vacant_list_broken = false;
        for (key, value) in iterator {
            if key < slab.entries.len() {
                // iterator is not sorted, might need to recreate vacant list
                if let Entry::Vacant(_) = slab.entries[key] {
                    vacant_list_broken = true;
                    slab.len += 1;
                }
                // if an element with this key already exists, replace it.
                // This is consisent with HashMap and BtreeMap
                slab.entries[key] = Entry::Occupied(value);
            } else {
                // insert holes as necessary
                while slab.entries.len() < key {
                    // add the entry to the start of the vacant list
                    let next = slab.next;
                    slab.next = slab.entries.len();
                    slab.entries.push(Entry::Vacant(next));
                }
                slab.entries.push(Entry::Occupied(value));
                slab.len += 1;
            }
        }
        if slab.len == slab.entries.len() {
            // no vacant enries, so next might not have been updated
            slab.next = slab.entries.len();
        } else if vacant_list_broken {
            slab.recreate_vacant_list();
        }
        slab
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

impl<T> fmt::Debug for IntoIter<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Iter")
            .field("curr", &self.curr)
            .field("remaining", &self.entries.len())
            .finish()
    }
}

impl<'a, T: 'a> fmt::Debug for Iter<'a, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Iter")
            .field("curr", &self.curr)
            .field("remaining", &self.entries.len())
            .finish()
    }
}

impl<'a, T: 'a> fmt::Debug for IterMut<'a, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("IterMut")
            .field("curr", &self.curr)
            .field("remaining", &self.entries.len())
            .finish()
    }
}

impl<'a, T: 'a> fmt::Debug for Drain<'a, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Drain").finish()
    }
}

// ===== VacantEntry =====

impl<'a, T> VacantEntry<'a, T> {
    pub fn insert(self, val: T) -> &'a mut T {
        self.slab.insert_at(self.key, val);

        match self.slab.entries.get_mut(self.key) {
            Some(&mut Entry::Occupied(ref mut v)) => v,
            _ => unreachable!(),
        }
    }

    pub fn key(&self) -> usize {
        self.key
    }
}

// ===== IntoIter =====

impl<T> Iterator for IntoIter<T> {
    type Item = (usize, T);

    fn next(&mut self) -> Option<(usize, T)> {
        while let Some(entry) = self.entries.next() {
            let curr = self.curr;
            self.curr += 1;

            if let Entry::Occupied(v) = entry {
                return Some((curr, v));
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.entries.len()))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<(usize, T)> {
        while let Some(entry) = self.entries.next_back() {
            if let Entry::Occupied(v) = entry {
                let key = self.curr + self.entries.len();
                return Some((key, v));
            }
        }

        None
    }
}

// ===== Iter =====

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<(usize, &'a T)> {
        while let Some(entry) = self.entries.next() {
            let curr = self.curr;
            self.curr += 1;

            if let Entry::Occupied(ref v) = *entry {
                return Some((curr, v));
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.entries.len()))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<(usize, &'a T)> {
        while let Some(entry) = self.entries.next_back() {
            if let Entry::Occupied(ref v) = *entry {
                let key = self.curr + self.entries.len();
                return Some((key, v));
            }
        }

        None
    }
}

// ===== IterMut =====

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (usize, &'a mut T);

    fn next(&mut self) -> Option<(usize, &'a mut T)> {
        while let Some(entry) = self.entries.next() {
            let curr = self.curr;
            self.curr += 1;

            if let Entry::Occupied(ref mut v) = *entry {
                return Some((curr, v));
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.entries.len()))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<(usize, &'a mut T)> {
        while let Some(entry) = self.entries.next_back() {
            if let Entry::Occupied(ref mut v) = *entry {
                let key = self.curr + self.entries.len();
                return Some((key, v));
            }
        }

        None
    }
}

// ===== Drain =====

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        while let Some(entry) = self.0.next() {
            if let Entry::Occupied(v) = entry {
                return Some(v);
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.0.len()))
    }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        while let Some(entry) = self.0.next_back() {
            if let Entry::Occupied(v) = entry {
                return Some(v);
            }
        }

        None
    }
}
