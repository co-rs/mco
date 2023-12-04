use std::sync::atomic::AtomicPtr;
use std::time::Duration;
use crate::detail::{initialize_call_frame, swap_registers, Registers};
use crate::gen_impl::DEFAULT_STACK_SIZE;
use crate::stack::{Stack, SysStack};

#[derive(Debug)]
pub struct RegContext {
    /// Hold the registers while the task or scheduler is suspended
    regs: Registers,
    stack: Option<*const Stack>,
}

// first argument is task handle, second is thunk ptr
pub type InitFn = fn(usize, *mut usize) -> !;

impl RegContext {
    pub fn empty() -> RegContext {
        RegContext {
            regs: Registers::new(),
            stack: Option::None,
        }
    }

    #[inline]
    pub fn prefetch(&self) {
        self.regs.prefetch();
    }

    /// Create a new context
    #[allow(dead_code)]
    pub fn new(init: InitFn, arg: usize, start: *mut usize, stack: &Stack) -> RegContext {
        let mut ctx = RegContext::empty();
        ctx.init_with(init, arg, start, stack);
        ctx
    }

    /// init the generator register
    #[inline]
    pub fn init_with(&mut self, init: InitFn, arg: usize, start: *mut usize, stack: &Stack) {
        // Save and then immediately load the current context,
        // which we will then modify to call the given function when restoredtack
        initialize_call_frame(&mut self.regs, init, arg, start, stack);
        self.stack = Some(stack);
    }

    /// Switch contexts
    ///
    /// Suspend the current execution context and resume another by
    /// saving the registers values of the executing thread to a Context
    /// then loading the registers from a previously saved Context.
    #[inline]
    pub fn swap(out_context: &mut RegContext, in_context: &RegContext) {
        // debug!("register raw swap");
        // in_context.stack_restore();
        // out_context.stack_restore();
        unsafe { swap_registers(&mut out_context.regs, &in_context.regs) }
        //in_context.stack_reduce();
    }

    /// Load the context and switch. This function will never return.
    #[inline]
    #[allow(dead_code)]
    pub fn load(to_context: &RegContext) {
        let mut cur = Registers::new();
        let regs: &Registers = &to_context.regs;

        unsafe { swap_registers(&mut cur, regs) }
    }

    // pub fn stack_restore(&self) {
    //     match self.stack.as_ref() {
    //         None => {}
    //         Some(v) => {
    //             let stack = unsafe { v.as_ref().unwrap() };
    //             if stack.size() < 10240 {
    //                 let mut data = stack.get_stack_data();
    //                 println!("before restore={}",data.len());
    //                 let left = 10240 - data.len();
    //                 for _ in 0..left {
    //                     data.insert(0, 0u8);
    //                 }
    //                 let mut idx = 0;
    //                 for x in &data {
    //                     if *x != 0 {
    //                         println!("stack_restore not zero={},{}",idx, data.len());
    //                         break;
    //                     }
    //                     idx += 1;
    //                 }
    //                 unsafe {
    //                     let mut new_stack = Stack::new(10240);
    //                     println!("restore {}", data.len());
    //                     new_stack.write_stack_data(data);
    //                     *stack.buf = new_stack.buf;
    //                 }
    //             }
    //         }
    //     }
    // }
    // #[inline]
    // pub fn stack_reduce(&self) {
    //     match self.stack.as_ref() {
    //         None => {}
    //         Some(v) => {
    //             let stack = unsafe { v.as_ref().unwrap() };
    //             let data = stack.get_stack_data();
    //
    //             if data.len() == 10240 {
    //                 let mut idx = 0;
    //                 for x in &data {
    //                     if *x != 0 {
    //                         println!("stack_reduce not zero={}，len={}", idx, stack.size());
    //                         break;
    //                     }
    //                     idx += 1;
    //                 }
    //
    //                 // stack.drop_stack();
    //                 let mut new_data = vec![];
    //                 for x in &data[idx..] {
    //                     new_data.push(*x);
    //                 }
    //                 if new_data.len() != new_data.len().next_power_of_two() {
    //                     let v = new_data.len().next_power_of_two() - new_data.len();
    //                     for _ in 0..v {
    //                         new_data.insert(0, 0u8);
    //                     }
    //                 }
    //                 unsafe {
    //                     let mut new_stack = Stack::new(new_data.len());
    //                     let before = new_data.len();
    //                     new_stack.write_stack_data(new_data);
    //                     println!("stack_reduce={} {}", before, new_stack.size());
    //                     //stack.drop_stack();
    //                     *stack.buf.lock() = new_stack.buf.into_inner();
    //                 }
    //             }
    //
    //             //println!("stack_reduce success");
    //         }
    //     }
    // }
}


#[cfg(test)]
mod test {
    use std::mem::transmute;

    use crate::reg_context::RegContext;
    use crate::stack::Stack;

    const MIN_STACK: usize = 1024;

    fn init_fn(arg: usize, f: *mut usize) -> ! {
        let func: fn() = unsafe { transmute(f) };
        func();

        let ctx: &RegContext = unsafe { transmute(arg) };
        RegContext::load(ctx);

        unreachable!("Should never comeback");
    }

    #[test]
    fn test_swap_context() {
        static mut VAL: bool = false;
        let mut cur = RegContext::empty();

        fn callback() {
            unsafe {
                VAL = true;
            }
        }

        let stk = Stack::new(MIN_STACK);
        let ctx = RegContext::new(
            init_fn,
            unsafe { transmute(&cur) },
            unsafe { transmute(callback as usize) },
            &stk,
        );

        RegContext::swap(&mut cur, &ctx);
        unsafe {
            assert!(VAL);
        }
    }

    #[test]
    fn stack_write_read_stack(){
        let mut new_stack = Stack::new(2048);
        let mut v =Vec::new();
        for _ in 0..2048{
            v.push(0u8);
        }
        v[2047]=1u8;
        new_stack.write_stack_data(v.clone());
        let stack_data=new_stack.get_stack_data();
        println!("new len= {}",stack_data.len());

        println!("st {} {}",stack_data[2047],stack_data[0]);

        assert_eq!(stack_data,v);
    }
}
