use crate::std::sync::{Mutex, MutexGuard};
use serde::ser::SerializeMap;
use serde::{Deserializer, Serialize, Serializer};
use std::borrow::Borrow;
use std::cell::UnsafeCell;
use std::collections::{
    hash_map::Iter as MapIter, hash_map::IterMut as MapIterMut, HashMap as Map,
};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub type SyncHashMap<K, V> = SyncHashMapImpl<K, V>;

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
pub struct SyncHashMapImpl<K: Eq + Hash + Clone, V> {
    read: UnsafeCell<Map<K, V>>,
    dirty: Mutex<Map<K, V>>,
}

impl<K: Eq + Hash + Clone, V> Drop for SyncHashMapImpl<K, V> {
    fn drop(&mut self) {
        unsafe {
            let k = (&mut *self.read.get()).keys().clone();
            for x in k {
                let v = (&mut *self.read.get()).remove(x);
                match v {
                    None => {}
                    Some(v) => {
                        std::mem::forget(v);
                    }
                }
            }
        }
    }
}

/// this is safety, dirty mutex ensure
unsafe impl<K: Eq + Hash + Clone, V> Send for SyncHashMapImpl<K, V> {}

/// this is safety, dirty mutex ensure
unsafe impl<K: Eq + Hash + Clone, V> Sync for SyncHashMapImpl<K, V> {}

//TODO maybe K will use transmute_copy replace Clone?
impl<K, V> SyncHashMapImpl<K, V>
where
    K: std::cmp::Eq + Hash + Clone,
{
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self::new())
    }

    pub fn new() -> Self {
        Self {
            read: UnsafeCell::new(Map::new()),
            dirty: Mutex::new(Map::new()),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            read: UnsafeCell::new(Map::with_capacity(capacity)),
            dirty: Mutex::new(Map::with_capacity(capacity)),
        }
    }

    pub fn insert(&self, k: K, v: V) -> Option<V>
    where
        K: Clone,
    {
        match self.dirty.lock() {
            Ok(mut m) => {
                let op = m.insert(k.clone(), v);
                match op {
                    None => {
                        let r = m.get(&k);
                        unsafe {
                            (&mut *self.read.get()).insert(k, std::mem::transmute_copy(r.unwrap()));
                        }
                        None
                    }
                    Some(v) => Some(v),
                }
            }
            Err(_) => Some(v),
        }
    }

    pub fn remove(&self, k: &K) -> Option<V>
    where
        K: Clone,
    {
        match self.dirty.lock() {
            Ok(mut m) => {
                let op = m.remove(k);
                match op {
                    Some(v) => {
                        unsafe {
                            let r = (&mut *self.read.get()).remove(k);
                            match r {
                                None => {}
                                Some(r) => {
                                    std::mem::forget(r);
                                }
                            }
                        }
                        Some(v)
                    }
                    None => None,
                }
            }
            Err(_) => None,
        }
    }

    pub fn len(&self) -> usize {
        unsafe { (&*self.read.get()).len() }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { (&*self.read.get()).is_empty() }
    }

    pub fn clear(&self) {
        match self.dirty.lock() {
            Ok(mut m) => {
                m.clear();
                unsafe {
                    let k = (&mut *self.read.get()).keys().clone();
                    for x in k {
                        let v = (&mut *self.read.get()).remove(x);
                        match v {
                            None => {}
                            Some(v) => {
                                std::mem::forget(v);
                            }
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }

    pub fn shrink_to_fit(&self) {
        match self.dirty.lock() {
            Ok(mut m) => {
                unsafe { (&mut *self.read.get()).shrink_to_fit() }
                m.shrink_to_fit()
            }
            Err(_) => {}
        }
    }

    pub fn from(map: Map<K, V>) -> Self
    where
        K: Clone + Eq + Hash,
    {
        let s = Self::with_capacity(map.capacity());
        match s.dirty.lock() {
            Ok(mut m) => {
                *m = map;
                unsafe {
                    for (k, v) in m.iter() {
                        (&mut *s.read.get()).insert(k.clone(), std::mem::transmute_copy(v));
                    }
                }
            }
            Err(_) => {}
        }
        s
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// Since reading a map is unlocked, it is very fast
    ///
    /// test bench_sync_hash_map_read   ... bench:           8 ns/iter (+/- 0)
    /// # Examples
    ///
    /// ```
    /// use mco::std::sync::{SyncHashMap};
    ///
    /// let map = SyncHashMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(*map.get(&1).unwrap(), "a");
    /// assert_eq!(map.get(&2).is_none(), true);
    /// ```
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        unsafe {
            let k = (&*self.read.get()).get(k);
            match k {
                None => None,
                Some(s) => Some(s),
            }
        }
    }

    pub fn get_mut<Q: ?Sized>(&self, k: &Q) -> Option<SyncHashMapRefMut<'_, K, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let g = self.dirty.lock();
        match g {
            Ok(m) => {
                let mut r = SyncHashMapRefMut { g: m, value: None };
                unsafe {
                    r.value = Some(change_lifetime_mut(r.g.get_mut(k)?));
                }
                Some(r)
            }
            Err(_) => None,
        }
    }

    pub fn iter(&self) -> MapIter<'_, K, V> {
        unsafe { (&*self.read.get()).iter() }
    }

    pub fn iter_mut(&self) -> IterHashMut<'_, K, V> {
        loop {
            match self.dirty.lock() {
                Ok(m) => {
                    let mut iter = IterHashMut { g: m, inner: None };
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

    pub fn into_iter(self) -> MapIter<'static, K, V> {
        unsafe { (&*self.read.get()).iter() }
    }
}

unsafe fn change_lifetime_mut<'a, 'b, T>(x: &'a mut T) -> &'b mut T {
    &mut *(x as *mut T)
}

pub struct SyncHashMapRefMut<'a, K, V> {
    g: MutexGuard<'a, Map<K, V>>,
    value: Option<&'a mut V>,
}

impl<'a, K, V> Deref for SyncHashMapRefMut<'_, K, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.value.as_ref().unwrap()
    }
}

impl<'a, K, V> DerefMut for SyncHashMapRefMut<'_, K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_mut().unwrap()
    }
}

impl<'a, K, V> Debug for SyncHashMapRefMut<'_, K, V>
where
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<'a, K, V> PartialEq<Self> for SyncHashMapRefMut<'_, K, V>
where
    V: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<'a, K, V> Eq for SyncHashMapRefMut<'_, K, V> where V: Eq {}

pub struct IterHash<'a, K, V> {
    inner: Option<MapIter<'a, K, *const V>>,
}

impl<'a, K, V> Iterator for IterHash<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.as_mut().unwrap().next();
        match next {
            None => None,
            Some((k, v)) => {
                if v.is_null() {
                    None
                } else {
                    unsafe { Some((k, &**v)) }
                }
            }
        }
    }
}

pub struct IterHashMut<'a, K, V> {
    g: MutexGuard<'a, Map<K, V>>,
    inner: Option<MapIterMut<'a, K, V>>,
}

impl<'a, K, V> Deref for IterHashMut<'a, K, V> {
    type Target = MapIterMut<'a, K, V>;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

impl<'a, K, V> DerefMut for IterHashMut<'a, K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}

impl<'a, K, V> Iterator for IterHashMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.as_mut().unwrap().next()
    }
}

impl<'a, K, V> IntoIterator for &'a SyncHashMapImpl<K, V>
where
    K: Eq + Hash + Clone,
{
    type Item = (&'a K, &'a V);
    type IntoIter = MapIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut SyncHashMapImpl<K, V>
where
    K: Eq + Hash + Clone,
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterHashMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V> IntoIterator for SyncHashMapImpl<K, V>
where
    K: Eq + Hash + Clone,
    K: 'static,
    V: 'static,
{
    type Item = (&'static K, &'static V);
    type IntoIter = MapIter<'static, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<K: Eq + Hash + Clone, V> From<Map<K, V>> for SyncHashMapImpl<K, V> {
    fn from(arg: Map<K, V>) -> Self {
        Self::from(arg)
    }
}

impl<K, V> serde::Serialize for SyncHashMapImpl<K, V>
where
    K: Eq + Hash + Clone + Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut m = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self.iter() {
            m.serialize_key(k)?;
            m.serialize_value(v)?;
        }
        m.end()
    }
}

impl<'de, K, V> serde::Deserialize<'de> for SyncHashMapImpl<K, V>
where
    K: Eq + Hash + Clone + serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let m = Map::deserialize(deserializer)?;
        Ok(Self::from(m))
    }
}

impl<K, V> Debug for SyncHashMapImpl<K, V>
where
    K: std::cmp::Eq + Hash + Clone + Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut m = f.debug_map();
        for (k, v) in self.iter() {
            m.key(k);
            m.value(v);
        }
        m.finish()
    }
}

#[cfg(test)]
mod test {
    use crate::coroutine::sleep;
    use crate::sleep;
    use crate::std::map::SyncHashMap;
    use crate::std::sync::WaitGroup;
    use std::collections::HashMap;
    use std::ops::Deref;
    use std::sync::atomic::Ordering;
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    pub fn test_debug() {
        let m: SyncHashMap<i32, i32> = SyncHashMap::new();
        m.insert(1, 1);
        println!("{:?}", m);
        assert_eq!(format!("{:?}", m), "{1: 1}");
    }

    #[test]
    pub fn test_empty() {
        let m: SyncHashMap<i32, i32> = SyncHashMap::new();
        assert_eq!(0, m.len());
    }

    #[test]
    pub fn test_insert() {
        let m = SyncHashMap::<i32, i32>::new();
        let insert = m.insert(1, 2);
        assert_eq!(insert.is_none(), true);
    }

    #[test]
    pub fn test_insert2() {
        let m = Arc::new(SyncHashMap::<String, String>::new());
        m.insert("/".to_string(), "1".to_string());
        m.insert("/js".to_string(), "2".to_string());
        m.insert("/fn".to_string(), "3".to_string());

        assert_eq!(&"1".to_string(), m.get("/").unwrap());
        assert_eq!(&"2".to_string(), m.get("/js").unwrap());
        assert_eq!(&"3".to_string(), m.get("/fn").unwrap());
    }

    #[test]
    pub fn test_insert3() {
        let m = Arc::new(SyncHashMap::<i32, i32>::new());
        let wg = WaitGroup::new();
        for _ in 0..100000 {
            let wg1 = wg.clone();
            let wg2 = wg.clone();
            let m1 = m.clone();
            let m2 = m.clone();
            co!(move || {
                m1.remove(&1);
                let insert = m1.insert(1, 2);
                drop(wg1);
            });
            co!(move || {
                m2.remove(&1);
                let insert = m2.insert(1, 2);
                drop(wg2);
            });
        }
        wg.wait();
    }

    #[test]
    pub fn test_insert4() {
        let m = Arc::new(SyncHashMap::<i32, i32>::new());
        let wg = WaitGroup::new();
        for _ in 0..8 {
            let wg1 = wg.clone();
            let wg2 = wg.clone();
            let m1 = m.clone();
            let m2 = m.clone();
            co!(move || {
                for i in 0..10000 {
                    m1.remove(&i);
                    let insert = m1.insert(i, i);
                }
                drop(wg1);
            });
            co!(move || {
                for i in 0..10000 {
                    m2.remove(&i);
                    let insert = m2.insert(i, i);
                }
                drop(wg2);
            });
        }
        wg.wait();
    }

    #[test]
    pub fn test_get() {
        let m = SyncHashMap::<i32, i32>::new();
        let insert = m.insert(1, 2);
        let g = m.get(&1).unwrap();
        assert_eq!(&2, g);
    }

    #[derive(Clone, Debug, Eq, PartialEq, Hash)]
    pub struct A {
        inner: i32,
    }

    impl Drop for A {
        fn drop(&mut self) {
            println!("droped");
        }
    }

    #[test]
    pub fn test_remove() {
        let a = A { inner: 0 };
        let m = SyncHashMap::<i32, A>::new();
        let insert = m.insert(1, a);
        let g = m.get(&1).unwrap();
        let rm = m.remove(&1).unwrap();
        println!("rm:{:?}", rm);
        drop(rm);
        assert_eq!(true, m.is_empty());
        assert_eq!(true, m.dirty.lock().unwrap().is_empty());
        assert_eq!(None, m.get(&1));
        assert_eq!(&A { inner: 0 }, g);
    }

    #[test]
    pub fn test_remove2() {
        let m = SyncHashMap::<i32, String>::new();
        for i in 0..1000000 {
            m.insert(i, String::from("safdfasdfasdfasdfasdfasdfsadf"));
        }
        sleep(Duration::from_secs(2));
        println!("start clean");
        m.clear();
        m.shrink_to_fit();
        println!("done,now you can see mem usage");
        sleep(Duration::from_secs(5));
        for i in 0..1000000 {
            m.insert(i, String::from("safdfasdfasdfasdfasdfasdfsadf"));
        }
        sleep(Duration::from_secs(2));
        println!("start clean");
        m.clear();
        m.shrink_to_fit();
        println!("done,now you can see mem usage");
        sleep(Duration::from_secs(5));
    }

    #[test]
    pub fn test_iter() {
        let m = SyncHashMap::<i32, i32>::new();
        let insert = m.insert(1, 2);
        for (k, v) in m.iter() {
            assert_eq!(*k, 1);
            assert_eq!(*v, 2);
        }
    }

    #[test]
    pub fn test_iter_mut() {
        let m = SyncHashMap::<i32, i32>::new();
        let insert = m.insert(1, 2);
        for (k, v) in m.iter_mut() {
            assert_eq!(*k, 1);
            assert_eq!(*v, 2);
        }
    }

    #[test]
    pub fn test_smoke2() {
        let wait1 = WaitGroup::new();
        let m1 = Arc::new(SyncHashMap::<i32, i32>::new());
        for i in 0..10000 {
            let wg = wait1.clone();
            let m = m1.clone();

            let wg2 = wait1.clone();
            let m2 = m1.clone();
            co!(move || {
                let insert = m.insert(i, i);
                let g = m.get(&i).unwrap();
                assert_eq!(i, *g.deref());
                drop(wg);
                println!("done{}", i);
            });
            co!(move || {
                let g = m2.remove(&i);
                if g.is_some() {
                    println!("done remove {}", i);
                    drop(wg2);
                }
            });
        }
        wait1.wait();
    }

    #[test]
    pub fn test_smoke3() {
        let wait1 = WaitGroup::new();
        let m1 = Arc::new(SyncHashMap::<i32, i32>::new());
        for mut i in 0..10000 {
            i = 1;
            let wg = wait1.clone();
            let m = m1.clone();
            co!(move || {
                let insert = m.insert(i, i);
                let g = m.get(&i).unwrap();
                assert_eq!(i, *g.deref());
                drop(wg);
                println!("done{}", i);
            });
            let wg2 = wait1.clone();
            let m2 = m1.clone();
            co!(move || {
                let g = m2.remove(&i);
                drop(wg2);
            });
        }
        wait1.wait();
    }
}
