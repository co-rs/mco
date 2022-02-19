use std::cell::UnsafeCell;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::slice::SliceIndex;
use std::sync::Arc;
use crate::std::sync::{Mutex, MutexGuard};

use std::slice::{Iter as SliceIter, IterMut as SliceIterMut};
use serde::{Deserializer, Serialize, Serializer};
use serde::ser::SerializeSeq;


pub type SyncVec<V> = SyncVecImpl<V>;


pub struct SyncVecImpl<V> {
    read: UnsafeCell<Vec<V>>,
    dirty: Mutex<Vec<V>>,
}

/// this is safety, dirty mutex ensure
unsafe impl<V> Send for SyncVecImpl<V> {}

/// this is safety, dirty mutex ensure
unsafe impl<V> Sync for SyncVecImpl<V> {}

impl<V> SyncVecImpl<V> {
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self::new())
    }

    pub fn new() -> Self {
        Self {
            read: UnsafeCell::new(Vec::new()),
            dirty: Mutex::new(Vec::new()),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            read: UnsafeCell::new(Vec::with_capacity(capacity)),
            dirty: Mutex::new(Vec::with_capacity(capacity)),
        }
    }

    pub fn insert(&self, index: usize, v: V) -> Option<V> {
        match self.dirty.lock() {
            Ok(mut m) => {
                m.insert(index, v);
                let len = m.len();
                unsafe {
                    let r = m.get_unchecked(len - 1);
                    (&mut *self.read.get()).insert(index, std::ptr::read(r));
                }
                None
            }
            Err(_) => {
                Some(v)
            }
        }
    }

    pub fn push(&self, v: V) -> Option<V> {
        match self.dirty.lock() {
            Ok(mut m) => {
                m.push(v);
                let len = m.len();
                unsafe {
                    let r = m.get_unchecked(len - 1);
                    (&mut *self.read.get()).push(std::ptr::read(r));
                }
                None
            }
            Err(_) => {
                Some(v)
            }
        }
    }

    pub fn pop(&self) -> Option<V> {
        match self.dirty.lock() {
            Ok(mut m) => {
                match m.pop() {
                    None => {
                        return None;
                    }
                    Some(s) => {
                        unsafe {
                            let r = (&mut *self.read.get()).pop();
                            match r{
                                None => {}
                                Some(r) => {
                                    std::mem::forget(r);
                                }
                            }
                        }
                        return Some(s);
                    }
                }
            }
            Err(_) => {
                None
            }
        }
    }

    pub fn remove(&self, index: usize) -> Option<V> {
        match self.get(index) {
            None => {
                None
            }
            Some(_) => {
                match self.dirty.lock() {
                    Ok(mut m) => {
                        let v = m.remove(index);
                        unsafe {
                            let r = (&mut *self.read.get()).remove(index);
                            std::mem::forget(r);
                        }
                        Some(v)
                    }
                    Err(_) => {
                        None
                    }
                }
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
                m.clear();
                unsafe {
                    loop {
                        match (&mut *self.read.get()).pop() {
                            None => {
                                break;
                            }
                            Some(v) => {
                                std::mem::forget(v)
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
                unsafe {
                    (&mut *self.read.get()).shrink_to_fit()
                }
                m.shrink_to_fit()
            }
            Err(_) => {}
        }
    }

    pub fn from(map: Vec<V>) -> Self {
        let s = Self::with_capacity(map.capacity());
        match s.dirty.lock() {
            Ok(mut m) => {
                *m = map;
                unsafe {
                    for v in m.iter() {
                        (&mut *s.read.get()).push(std::ptr::read(v));
                    }
                }
            }
            Err(_) => {}
        }
        s
    }

    pub fn get(&self, index: usize) -> Option<&V>
    {
        unsafe {
            let k = (&*self.read.get()).get(index);
            match k {
                None => { None }
                Some(s) => {
                    Some(s)
                }
            }
        }
    }

    pub unsafe fn get_uncheck(&self, index: usize) -> Option<&V>
    {
        unsafe {
            let k = (&*self.read.get()).get_unchecked(index);
            Some(k)
        }
    }

    pub fn get_mut(&self, index: usize) -> Option<VecRefMut<'_, V>>
    {
        let g = self.dirty.lock();
        match g {
            Ok(m) => {
                let mut r = VecRefMut {
                    g: m,
                    value: None,
                };
                unsafe {
                    r.value = Some(change_lifetime_mut(r.g.get_mut(index)?));
                }
                Some(r)
            }
            Err(_) => {
                None
            }
        }
    }

    pub fn iter(&self) -> SliceIter<'_, V> {
        unsafe {
            (&*self.read.get()).iter()
        }
    }

    pub fn iter_mut(&self) -> IterMut<'_, V> {
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

    pub fn into_iter(self) -> SliceIter<'static, V> {
        unsafe {
            (&*self.read.get()).iter()
        }
    }
}


pub unsafe fn change_lifetime_const<'a, 'b, T>(x: &'a T) -> &'b T {
    &*(x as *const T)
}

pub unsafe fn change_lifetime_mut<'a, 'b, T>(x: &'a mut T) -> &'b mut T {
    &mut *(x as *mut T)
}

pub struct VecRefMut<'a, V> {
    g: MutexGuard<'a, Vec<V>>,
    value: Option<&'a mut V>,
}

impl<'a, V> Deref for VecRefMut<'_, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.value.as_ref().unwrap()
    }
}

impl<'a, V> DerefMut for VecRefMut<'_, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_mut().unwrap()
    }
}

impl<'a, V> Debug for VecRefMut<'_, V> where V: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}


pub struct Iter<'a, V> {
    inner: Option<SliceIter<'a, *const V>>,
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.as_mut().unwrap().next();
        match next {
            None => { None }
            Some(v) => {
                if v.is_null() {
                    None
                } else {
                    unsafe {
                        Some(&**v)
                    }
                }
            }
        }
    }
}


pub struct IterMut<'a, V> {
    g: MutexGuard<'a, Vec<V>>,
    inner: Option<SliceIterMut<'a, V>>,
}

impl<'a, V> Deref for IterMut<'a, V> {
    type Target = SliceIterMut<'a, V>;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

impl<'a, V> DerefMut for IterMut<'a, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}

impl<'a, V> Iterator for IterMut<'a, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.as_mut().unwrap().next()
    }
}


impl<'a, V> IntoIterator for &'a SyncVecImpl<V> {
    type Item = &'a V;
    type IntoIter = SliceIter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


impl<'a, V> IntoIterator for &'a mut SyncVecImpl<V> {
    type Item = &'a mut V;
    type IntoIter = IterMut<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<V> IntoIterator for SyncVecImpl<V> where V: 'static {
    type Item = &'static V;
    type IntoIter = SliceIter<'static, V>;

    fn into_iter(mut self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<V> serde::Serialize for SyncVecImpl<V> where V: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut m = serializer.serialize_seq(Some(self.len()))?;
        for v in self.iter() {
            m.serialize_element(v);
        }
        m.end()
    }
}

impl<'de, V> serde::Deserialize<'de> for SyncVecImpl<V> where V: serde::Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let m = Vec::deserialize(deserializer)?;
        Ok(Self::from(m))
    }
}

impl<V> Debug for SyncVecImpl<V> where V: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut m = f.debug_list();
        for v in self.iter() {
            m.entry(v);
        }
        m.finish()
    }
}


#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::sync::Arc;
    use std::sync::atomic::{Ordering};
    use std::time::Duration;
    use crate::coroutine::sleep;
    use crate::sleep;
    use crate::std::vec::SyncVec;
    use crate::std::sync::{WaitGroup};

    #[test]
    pub fn test_debug() {
        let m: SyncVec<i32> = SyncVec::new();
        m.push(1);
        println!("{:?}", m);
        assert_eq!(format!("{:?}", m), "[1]");
    }

    #[test]
    pub fn test_empty() {
        let m: SyncVec<i32> = SyncVec::new();
        assert_eq!(0, m.len());
    }

    #[test]
    pub fn test_push() {
        let m = SyncVec::<i32>::new();
        let insert = m.push(1);
        assert_eq!(insert.is_none(), true);
    }

    #[test]
    pub fn test_push2() {
        let m = Arc::new(SyncVec::<String>::new());
        m.push("1".to_string());
        m.push("2".to_string());
        m.push("3".to_string());

        assert_eq!(&"1".to_string(), m.get(0).unwrap());
        assert_eq!(&"2".to_string(), m.get(1).unwrap());
        assert_eq!(&"3".to_string(), m.get(2).unwrap());
    }

    #[test]
    pub fn test_insert3() {
        let m = Arc::new(SyncVec::<i32>::new());
        let wg = WaitGroup::new();
        for _ in 0..100000 {
            let wg1 = wg.clone();
            let wg2 = wg.clone();
            let m1 = m.clone();
            let m2 = m.clone();
            co!(move ||{
                 m1.pop();
                 let insert = m1.push( 2);
                 drop(wg1);
            });
            co!(move ||{
                 m2.pop();
                 let insert = m2.push( 2);
                 drop(wg2);
            });
        }
        wg.wait();
    }

    #[test]
    pub fn test_insert4() {
        let m = Arc::new(SyncVec::<i32>::new());
        let wg = WaitGroup::new();
        for _ in 0..8 {
            let wg1 = wg.clone();
            let wg2 = wg.clone();
            let m1 = m.clone();
            let m2 = m.clone();
            co!(move ||{
                 for i in 0..10000{
                     m1.pop();
                     let insert = m1.push( i);
                 }
                 drop(wg1);
            });
            co!(move ||{
                 for i in 0..10000{
                     m2.pop();
                     let insert = m2.push( i);
                 }
                 drop(wg2);
            });
        }
        wg.wait();
    }

    #[test]
    pub fn test_get() {
        let m = SyncVec::<i32>::new();
        let insert = m.push(2);
        let g = m.get(0).unwrap();
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
        let m = SyncVec::<A>::new();
        let insert = m.push(a);
        let g = m.get(0).unwrap();
        let rm = m.remove(0).unwrap();
        println!("rm:{:?}", rm);
        drop(rm);
        assert_eq!(true, m.is_empty());
        assert_eq!(true, m.dirty.lock().unwrap().is_empty());
        assert_eq!(None, m.get(0));
        assert_eq!(&A { inner: 0 }, g);
    }

    #[test]
    pub fn test_remove2() {
        let m = SyncVec::<String>::new();
        for i in 0..1000000 {
            m.push(String::from("safdfasdfasdfasdfasdfasdfsadf"));
        }
        sleep(Duration::from_secs(2));
        println!("start clean");
        m.clear();
        m.shrink_to_fit();
        println!("done,now you can see mem usage");
        sleep(Duration::from_secs(5));
        for i in 0..1000000 {
            m.push(String::from("safdfasdfasdfasdfasdfasdfsadf"));
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
        let m = SyncVec::<i32>::new();
        let insert = m.push(2);
        for v in m.iter() {
            assert_eq!(*v, 2);
        }
    }

    #[test]
    pub fn test_iter_mut() {
        let m = SyncVec::<i32>::new();
        let insert = m.push(2);
        for v in m.iter_mut() {
            assert_eq!(*v, 2);
        }
    }
}