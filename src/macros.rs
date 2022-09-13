/// macro used to spawn a coroutine
///
/// this macro is just a convenient wrapper for [`spawn`].
///
/// [`spawn`]: coroutine/fn.spawn.html
#[macro_export]
macro_rules! co {
    // for free spawn
    ($func:expr) => {{
        $crate::coroutine::spawn($func)
    }};

    // for builder/scope spawn
    ($builder:expr, $func:expr) => {{
        use $crate::coroutine::Spawn;
        unsafe { $builder.spawn($func) }
    }};

    // for cqueue add spawn
    ($cqueue:expr, $token:expr, $func:expr) => {{
        unsafe { $cqueue.add($token, $func) }
    }};
}

/// macro used to spawn a coroutine
///
/// this macro is just a convenient wrapper for [`spawn`].
///
/// [`spawn`]: coroutine/fn.spawn.html
#[macro_export]
macro_rules! spawn {
    // for free spawn
    ($func:expr) => {{
        unsafe { $crate::coroutine::spawn($func) }
    }};

    // for builder/scope spawn
    ($builder:expr, $func:expr) => {{
        use $crate::coroutine::Spawn;
        unsafe { $builder.spawn($func) }
    }};

    // for cqueue add spawn
    ($cqueue:expr, $token:expr, $func:expr) => {{
        unsafe { $cqueue.add($token, $func) }
    }};
}

/// macro used to spawn a coroutine with options such as name, stack_size.
///
/// this macro is just a convenient wrapper for [`spawn`].
/// However the supplied coroutine block is not wrapped in `unsafe` block
///
/// [`spawn`]: coroutine/fn.spawn.html
#[macro_export]
macro_rules! spawn_with {
    // for stack_size
    ($stack_size:expr, $func:expr) => {{
        fn _go_check<F, T>(stack_size: usize, f: F) -> F
        where
            F: FnOnce() -> T + Send + 'static,
            T: Send + 'static,
        {
            f
        }
        let f = _go_check($stack_size, $func);
        let builder = $crate::coroutine::Builder::new().stack_size($stack_size);
        unsafe { builder.spawn(f) }
    }};

    // for name and stack_size
    ($name: expr, $stack_size:expr, $func:expr) => {{
        fn _go_check<F, T>(name: &str, stack_size: usize, f: F) -> F
        where
            F: FnOnce() -> T + Send + 'static,
            T: Send + 'static,
        {
            f
        }
        let f = _go_check($name, $stack_size, $func);
        let builder = $crate::coroutine::Builder::new()
            .name($name.to_owned())
            .stack_size($stack_size);
        unsafe { builder.spawn(f) }
    }};
}

/// macro used to create the select coroutine
/// that will run in a infinite loop, and generate
/// as many events as possible
#[macro_export]
macro_rules! cqueue_add {
    ($cqueue:ident, $token:expr, $name:pat = $top:expr => $bottom:expr) => {{
        $crate::co!($cqueue, $token, |es| loop {
            let $name = $top;
            es.send(es.get_token());
            $bottom
        })
    }};
}

/// macro used to create the select coroutine
/// that will run only once, thus generate only one event
/// use mco::select;
///
#[macro_export]
macro_rules! cqueue_add_oneshot {
    ($cqueue:ident, $token:expr, $name:pat = $top:expr => $bottom:expr) => {{
        $crate::co!($cqueue, $token, |es| {
            if let $name = $top {
                $bottom
            }
            es.send(es.get_token());
        })
    }};
}

/// macro used to select for only one event
/// it will return the index of which event happens first
/// for example:
/// ```rust
/// use mco::{chan, select};
///
///     let (s, r) = chan!();
///     s.send(1);
///     select! {
///         rv = r.recv() => {
///             println!("{:?}",rv);
///         },
///         Ok(msg) = r.try_recv() => {
///             println!("{}",msg);
///         }
///     };
/// ```
#[macro_export]
macro_rules! select {
    (
        $($name:pat = $top:expr => $bottom:expr), +$(,)?
    ) => ($crate::select_token!($($name = $top => $bottom), +););
}
/// macro used to select for only one event
/// it will return the index of which event happens first
/// for example:
/// ```rust
/// use mco::{chan, select_token};
///
///     let (s, r) = chan!();
///     s.send(1);
///     let id = select_token! {
///         rv = r.recv() => {
///             println!("{:?}",rv);
///         }
///     };
/// ```
#[macro_export]
macro_rules! select_token {
    (
        $($name:pat = $top:expr => $bottom:expr), +$(,)?
    ) => ({
        $crate::cqueue::scope(|cqueue| {
            let mut _token = 0;
            $(
                $crate::cqueue_add_oneshot!(cqueue, _token, $name = $top => $bottom);
                _token += 1;
            )+
            match cqueue.poll(None) {
                Ok(ev) => return ev.token,
                _ => unreachable!("select error"),
            }
        })
    });
}

/// macro used to join all scoped sub coroutines
/// for example:
/// ```rust
/// use mco::join;
/// join!({  },
///       {  },
///       {  }
/// );
/// ```
#[macro_export]
macro_rules! join {
    (
        $($body:expr),+
    ) => ({
        use $crate::coroutine;
        coroutine::scope(|s| {
            $(
                $crate::co!(s, || $body);
            )+
        })
    })
}

/// A macro to create a `static` of type `LocalKey`
///
/// This macro is intentionally similar to the `thread_local!`, and creates a
/// `static` which has a `with` method to access the data on a coroutine.
///
/// The data associated with each coroutine local is per-coroutine,
/// so different coroutines will contain different values.
/// for example:
/// ```
///     mco::coroutine_local!(static FOO: i32 = 3);
///
///     // can only be called in coroutine context
///     FOO.with(|f| {
///         assert_eq!(*f, 3);
///     });
///
/// ```
#[macro_export]
macro_rules! coroutine_local {
    (static $NAME:ident : $t:ty = $e:expr) => {
        static $NAME: $crate::LocalKey<$t> = {
            fn __init() -> $t {
                $e
            }
            fn __key() -> ::std::any::TypeId {
                struct __A;
                ::std::any::TypeId::of::<__A>()
            }
            $crate::LocalKey {
                __init: __init,
                __key: __key,
            }
        };
    };
}
