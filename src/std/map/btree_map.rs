use std::borrow::{Borrow, BorrowMut};
use std::cell::UnsafeCell;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::{Arc, LockResult};
use std::time::Duration;
use crate::std::sync::{Mutex, MutexGuard};
use std::marker::PhantomData;

use std::collections::{BTreeMap as Map, btree_map::IntoIter as IntoIter, btree_map::Iter as MapIter, btree_map::IterMut as MapIterMut};

pub type SyncBtreeMap<K, V> = SyncMapImpl<K, V>;

/// this sync map used to many reader,writer less.space-for-time strategy
///
/// Map is like a Go map[interface{}]interface{} but is safe for concurrent use
/// by multiple goroutines without additional locking or coordination.
/// Loads, stores, and deletes run in amortized constant time.
///
/// The Map type is specialized. Most code should use a plain Go map instead,
/// with separate locking or coordination, for better type safety and to make it
/// easier to maintain other invariants along with the map content.
///
/// The Map type is optimized for two common use cases: (1) when the entry for a given
/// key is only ever written once but read many times, as in caches that only grow,
/// or (2) when multiple goroutines read, write, and overwrite entries for disjoint
/// sets of keys. In these two cases, use of a Map may significantly reduce lock
/// contention compared to a Go map paired with a separate Mutex or RWMutex.
///
/// The zero Map is empty and ready for use. A Map must not be copied after first use.
pub struct SyncMapImpl<K, V> {
    read: UnsafeCell<Map<K, *const V>>,
    dirty: Mutex<Map<K, V>>,
}

/// this is safety, dirty mutex ensure
unsafe impl<K, V> Send for SyncMapImpl<K, V> {}

/// this is safety, dirty mutex ensure
unsafe impl<K, V> Sync for SyncMapImpl<K, V> {}

impl<K, V> SyncMapImpl<K, V> where K: std::cmp::Eq + Hash + Clone {
    pub fn new() -> Self {
        Self {
            read: UnsafeCell::new(Map::new()),
            dirty: Mutex::new(Map::new()),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self::new()
    }

    pub fn insert(&self, k: K, v: V) -> Option<V> where K: Clone + std::cmp::Ord {
        match self.dirty.lock() {
            Ok(mut m) => {
                let op = m.insert(k.clone(), v);
                match op {
                    None => {
                        let r = m.get(&k);
                        unsafe {
                            (&mut *self.read.get()).insert(k, r.unwrap());
                        }
                        None
                    }
                    Some(v) => {
                        Some(v)
                    }
                }
            }
            Err(_) => {
                Some(v)
            }
        }
    }

    pub fn remove(&self, k: &K) -> Option<V> where K: Clone + std::cmp::Ord {
        match self.dirty.lock() {
            Ok(mut m) => {
                let op = m.remove(k);
                match op {
                    Some(v) => {
                        unsafe {
                            (&mut *self.read.get()).remove(k);
                        }
                        Some(v)
                    }
                    None => {
                        None
                    }
                }
            }
            Err(_) => {
                None
            }
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            (&*self.read.get()).len()
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            (&*self.read.get()).is_empty()
        }
    }

    pub fn clear(&self) {
        match self.dirty.lock() {
            Ok(mut m) => {
                unsafe {
                    (&mut *self.read.get()).clear()
                }
                m.clear()
            }
            Err(_) => {}
        }
    }

    pub fn shrink_to_fit(&self) {
        //nothing to do
    }

    /// # Examples
    ///
    /// ```
    /// use cogo::std::sync::{SyncBtreeMap};
    ///
    /// let map = SyncBtreeMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(*map.get(&1).unwrap(), "a");
    /// assert_eq!(map.get(&2).is_none(), true);
    /// ```
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<SyncMapRef<'_, V>>
        where
            K: Borrow<Q> + std::cmp::Ord,
            Q: Hash + Eq + std::cmp::Ord,
    {
        unsafe {
            let k = (&*self.read.get()).get(k);
            match k {
                None => { None }
                Some(s) => {
                    if s.is_null() {
                        return None;
                    }
                    Some(SyncMapRef {
                        value: Some(&**s)
                    })
                }
            }
        }
    }

    pub fn get_mut<Q: ?Sized>(&self, k: &Q) -> Option<SyncMapRefMut<'_, K, V>>
        where
            K: Borrow<Q> + std::cmp::Ord,
            Q: Hash + Eq + std::cmp::Ord,
    {
        let g = self.dirty.lock();
        match g {
            Ok(m) => {
                let mut r = SyncMapRefMut {
                    g: m,
                    value: None,
                };
                unsafe {
                    r.value = Some(change_lifetime_mut(r.g.get_mut(k)?));
                }
                Some(r)
            }
            Err(_) => {
                None
            }
        }
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        unsafe {
            let iter = (&*self.read.get()).iter();
            Iter {
                inner: Some(iter)
            }
        }
    }

    pub fn iter_mut(&self) -> IterMut<'_, K, V> {
        loop {
            match self.dirty.lock() {
                Ok(m) => {
                    let mut iter = IterMut {
                        g: m,
                        inner: None,
                    };
                    unsafe {
                        iter.inner = Some(change_lifetime_mut(&mut iter.g).iter_mut());
                    }
                    return iter;
                }
                Err(_) => {
                    continue;
                }
            }
        }
    }

    pub fn into_iter(self) -> Iter<'static, K, V> {
        unsafe {
            let iter = (&*self.read.get()).iter();
            Iter {
                inner: Some(iter)
            }
        }
    }
}

pub unsafe fn change_lifetime_const<'a, 'b, T>(x: &'a T) -> &'b T {
    &*(x as *const T)
}

pub unsafe fn change_lifetime_mut<'a, 'b, T>(x: &'a mut T) -> &'b mut T {
    &mut *(x as *mut T)
}


pub struct SyncMapRef<'a, V> {
    value: Option<&'a V>,
}

impl<V> Deref for SyncMapRef<'_, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.value.as_ref().unwrap()
    }
}

impl<V> Debug for SyncMapRef<'_, V> where V: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.unwrap().fmt(f)
    }
}


impl<V> PartialEq<Self> for SyncMapRef<'_, V> where V: Eq {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<V> Eq for SyncMapRef<'_, V> where V: Eq {}


pub struct SyncMapRefMut<'a, K, V> {
    g: MutexGuard<'a, Map<K, V>>,
    value: Option<&'a mut V>,
}


impl<'a, K, V> Deref for SyncMapRefMut<'_, K, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.value.as_ref().unwrap()
    }
}

impl<'a, K, V> DerefMut for SyncMapRefMut<'_, K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_mut().unwrap()
    }
}

impl<'a, K, V> Debug for SyncMapRefMut<'_, K, V> where V: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}


impl<'a, K, V> PartialEq<Self> for SyncMapRefMut<'_, K, V> where V: Eq {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<'a, K, V> Eq for SyncMapRefMut<'_, K, V> where V: Eq {}


pub struct Iter<'a, K, V> {
    inner: Option<MapIter<'a, K, *const V>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.as_mut().unwrap().next();
        match next {
            None => { None }
            Some((k, v)) => {
                if v.is_null() {
                    None
                } else {
                    unsafe {
                        Some((k, &**v))
                    }
                }
            }
        }
    }
}

pub struct IterMut<'a, K, V> {
    g: MutexGuard<'a, Map<K, V>>,
    inner: Option<MapIterMut<'a, K, V>>,
}

impl<'a, K, V> Deref for IterMut<'a, K, V> {
    type Target = MapIterMut<'a, K, V>;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

impl<'a, K, V> DerefMut for IterMut<'a, K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.as_mut().unwrap().next()
    }
}

impl<'a, K, V> IntoIterator for &'a SyncMapImpl<K, V> where K: Eq + Hash + Clone {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


impl<'a, K, V> IntoIterator for &'a mut SyncMapImpl<K, V> where K: Eq + Hash + Clone {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V> IntoIterator for SyncMapImpl<K, V> where
    K: Eq + Hash + Clone,
    K: 'static, V: 'static {
    type Item = (&'static K, &'static V);
    type IntoIter = Iter<'static, K, V>;

    fn into_iter(mut self) -> Self::IntoIter {
        self.into_iter()
    }
}


#[cfg(test)]
mod test {
    use std::collections::BTreeMap;
    use std::ops::Deref;
    use std::sync::Arc;
    use std::sync::atomic::{Ordering};
    use crate::std::map::SyncBtreeMap;
    use crate::std::sync::{WaitGroup};


    #[test]
    pub fn test_empty() {
        let m: SyncBtreeMap<i32, i32> = SyncBtreeMap::new();
        assert_eq!(0, m.len());
    }

    #[test]
    pub fn test_insert() {
        let m = SyncBtreeMap::<i32, i32>::new();
        let insert = m.insert(1, 2);
        assert_eq!(insert.is_none(), true);
    }

    #[test]
    pub fn test_get() {
        let m = SyncBtreeMap::<i32, i32>::new();
        let insert = m.insert(1, 2);
        let g = m.get(&1).unwrap();
        assert_eq!(2, *g.deref());
    }

    #[test]
    pub fn test_iter() {
        let m = SyncBtreeMap::<i32, i32>::new();
        let insert = m.insert(1, 2);
        for (k, v) in m.iter() {
            assert_eq!(*k, 1);
            assert_eq!(*v, 2);
        }
    }

    #[test]
    pub fn test_iter_mut() {
        let m = SyncBtreeMap::<i32, i32>::new();
        let insert = m.insert(1, 2);
        for (k, v) in m.iter_mut() {
            assert_eq!(*k, 1);
            assert_eq!(*v, 2);
        }
    }
}