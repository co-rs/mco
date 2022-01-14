pub mod hash_map;
pub mod btree_map;

pub use hash_map::*;
pub use btree_map::*;

/// make an hash map
/// for example:
/// let m = hash_map!{
///         1:1,
///         2:2,
///         f:v,
///         6:9*2
///     };
#[macro_export]
macro_rules! hash_map {
    { $($key:tt:$value:expr),+   $(,)?} => {
       {
            let mut temp_table_data = std::collections::hash_map::HashMap::with_capacity(0);
            $(temp_table_data.insert($key,$value);)+
            temp_table_data
        }
    };
}

#[macro_export]
macro_rules! btree_map {
    { $($key:tt:$value:expr),+   $(,)?} => {
       {
            let mut temp_table_data = std::collections::btree_map::BTreeMap::new();
             $(temp_table_data.insert($key,$value);)+
            temp_table_data
        }
    };
}