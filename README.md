# mco

<img style="width: 100px;height: 100px;" width="100" height="100" src="docs/imgs/logo.png" />

mco is a high-performance library for programming stackful coroutines with which you can easily develop and maintain
massive concurrent programs. It can be thought as the Rust version of the popular [Goroutine][go].

# way mco?
* Elegant coding,No need for async await
* Simple concurrency(CSP model), learning Golang
* Default MAX 6MB Stack Size(Reuse the stack owned by threads)
* many std like API

> Initial code frok from [May](https://github.com/Xudong-Huang/may) and we add Many improvements(Inspired by [Golang](https://golang.google.cn/),  [parking_lot](https://github.com/Amanieu/parking_lot)  and [crossbeam](https://github.com/crossbeam-rs/crossbeam)) and more...


# mco crates

> mco Powerful standard library

* ``` mco/std/queue ``` Basic queue data structures
* ``` mco/std/sync ```  Includes ``` Mutex/RwLock/WaitGroup/Semphore/chan!()/chan!(1000) ```...and more..
* ``` mco/std/defer ``` Defers evaluation of a block of code until the end of the scope.
* ``` mco/std/map ```  Provides the same concurrency map as Golang, with ```SyncHashMap``` and ```SyncBtreeMap```.It is
  suitable for concurrent environments with too many reads and too few writes
* ``` mco/std/vec ```  Provides the same concurrency vec
* ``` mco/std/time ``` Improve the implementation of a high performance time
* ``` mco/std/lazy ``` Thread/coroutine safe global variable,Lazy struct,OnceCell

> Crates based on mco implementation

* [mco-http](https://github.com/co-rs/mco-http) High-performance coroutine HTTP server and client
* [cdbc](https://github.com/co-rs/cdbc) Database Drivers include mysql, Postgres, AND SQLite
* [fast_log](https://github.com/co-rs/fast_log) High-performance log impl
* [mco-redis](https://github.com/co-rs/mco-redis) Redis client for mco
* [mco-redis-rs](https://github.com/co-rs/mco-redis-rs) fork from ```redis-rs``` Replace only TcpStream with MCO ::TcpStream
* [mco-rpc](https://github.com/co-rs/mco-rpc)  rpc server/client. support bincode/json rpc

## Features

* The stackful coroutine implementation is based on [generator][generator];
* Support schedule on a configurable number of threads for multi-core systems;
* Support coroutine version of a local storage ([CLS][cls]);
* Support efficient asynchronous network I/O;
* Support efficient timer management;
* Support standard synchronization primitives, a semaphore, an MPMC channel, etc;
* Support cancellation of coroutines;
* Support graceful panic handling that will not affect other coroutines;
* Support scoped coroutine creation;
* Support general selection for all the coroutine API;
* All the coroutine API are compatible with the standard library semantics;
* All the coroutine API can be safely called in multi-threaded context;
* Both stable, beta, and nightly channels are supported;
* x86_64 GNU/Linux, x86_64 Windows, x86_64 Mac, aarch64 Linux OS are supported.

* Support High performance chan(like golang)
* Support WaitGroup Support(like golang)
* Support defer!() (like golang)
* Support Rustls
* Support Time (like golang)
* Support error/err!() (like golang)
* Support select match Ok(v)/Err(e)  (like golang)
* Support Lazy/OnceCell
* Support SyncMap(like golang)
* Support Ticker(like golang)

## Usage
```toml
mco = "0.1"
```
A naive echo server implemented with mco:

```rust
#[macro_use]
extern crate mco;

use mco::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    while let Ok((mut stream, _)) = listener.accept() {
        go!(move || {
            let mut buf = vec![0; 1024 * 16]; // alloc in heap!
            while let Ok(n) = stream.read(&mut buf) {
                if n == 0 {
                    break;
                }
                stream.write_all(&buf[0..n]).unwrap();
            }
        });
    }
}

```

## More examples

### The I/O heavy bound examples

* [An echo server](examples/src/echo.rs)
* [An echo client](examples/src/echo_client.rs)
* [simple HTTP](examples/src/http.rs)
* [simple HTTPS](examples/src/https.rs)
* [tiny  HTTP](https://github.com/co-rs/mco-http)
* [WebSockets](examples/src/websocket.rs)

## Caveat

There is a detailed [document][caveat] that describes mco's main restrictions. In general, there are four things you
should follow when writing programs that use coroutines:

* Don't call thread-blocking API (It will hurt the performance);
* Carefully use Thread Local Storage (access TLS in coroutine might trigger undefined behavior).

> It's considered **unsafe** with the following pattern:
> ```rust
> set_tls();
> // Or another coroutine API that would cause scheduling:
> yield_now(); 
> use_tls();
> ```
> but it's **safe** if your code is not sensitive about the previous state of TLS. Or there is no coroutines scheduling between **set** TLS and **use** TLS.

* Don't run CPU bound tasks for long time, but it's ok if you don't care about fairness;
* In most modern operating systems, when starting a process, the standard Thread stack size is usually 8 MB, and mco provides a maximum stack space of 6MB. Typically, operating systems load memory pages on demand, such as starting about 1 million processes on my Mac/Unix system, which requires 15GB of memory space. 
* Don't exceed the coroutine stack. There is a guard page for each coroutine stack. When stack overflow occurs, it will
  trigger segment fault error.

**Note:**
> The first three rules are common when using cooperative asynchronous libraries in Rust. Even using a futures-based system also have these limitations. So what you should really focus on is a coroutine stack size, make sure it's big enough for your applications.

## How to tune a stack size

```rust
mco::config().set_stack_size(6*1024*1024);
```

* We are in urgent need of financial support and cooperation, welcome to contact us
* email: zhuxiujia@qq.com
* wechat: zxj347284221
> 捐赠

<img style="width: 400px;height: 600px;" width="400" height="600" src="https://raw.githubusercontent.com/rbatis/rbatis.io/master/docs/_media/wx_account.png" alt="zxj347284221" />

> 联系方式(添加好友请备注'mco') 微信群：先加微信，然后拉进群

<img style="width: 400px;height: 500px;" width="400" height="500" src="https://raw.githubusercontent.com/rbatis/rbatis.io/master/docs/_media/wechat.jpg" alt="zxj347284221" />
