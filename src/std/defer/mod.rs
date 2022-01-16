/// Defers evaluation of a block of code until the end of the scope.
/// Sort of LIFO(last-in, first-out queue)
///
///
/// for example:
/// ```
///  use cogo::defer;
///  //LIFO, so it will print: guard: 3  guard: 2   guard: 1
///  fn main(){
///     defer!({
///        println!("guard: 1");
///        });
///     defer!(||{
///        println!("guard: 2");
///        });
///     defer!{
///        println!("guard: 3");
///     }
/// }
/// ```
///
///
#[macro_export]
macro_rules! defer {
    ($func:block) => {
       let _guard = {
            pub struct Guard<F: FnOnce()>(Option<F>);
            impl<F: FnOnce()> Drop for Guard<F> {
                fn drop(&mut self) {
                    if let Some(f) = (self.0).take() {
                        f()
                    }
                }
            }
            Guard(Some(||$func))
        };
    };
    ($func:expr) => {
       let _guard = {
            pub struct Guard<F: FnOnce()>(Option<F>);
            impl<F: FnOnce()> Drop for Guard<F> {
                fn drop(&mut self) {
                    if let Some(f) = (self.0).take() {
                        f()
                    }
                }
            }
            Guard(Some($func))
        };
    };
    { $($func:expr;)+ } => {
       let _guard = {
            pub struct Guard<F: FnOnce()>(Option<F>);
            impl<F: FnOnce()> Drop for Guard<F> {
                fn drop(&mut self) {
                    if let Some(f) = (self.0).take() {
                        f()
                    }
                }
            }
            Guard(Some(||{
                $($func;)+
            }))
        };
        ;
    }
}