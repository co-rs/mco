/// Defers evaluation of a block of code until the end of the scope.
/// Sort of LIFO(last-in, first-out queue)
///
///
/// for example:
///  // will print:  None Exception! \n   guard: 2 \n  guard: 1
///  fn main(){
///     defer! {
///         println!("guard: 1");
///     }
///     defer! {
///         println!("guard: 2");
///     }
///     panic!("None Exception!");
///   }
///
///
///
#[macro_export]
macro_rules! defer {
    {$($body:stmt;)+} => {
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
                $($body)+
            }))
        };
    };
}