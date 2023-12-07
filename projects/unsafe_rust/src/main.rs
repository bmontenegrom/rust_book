use std::slice;

static mut COUNTER: u32 = 0;

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    println!("a = {:?}, b = {:?}", a, b);

    let (a, b) = split_at_mut(r, 3);
    println!("a = {:?}, b = {:?}", a, b);

    unsafe{
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }


    add_to_count(3);

    unsafe{
        println!("COUNTER = {COUNTER}");
    }
}

unsafe fn dangerous() {}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {//use function from C
    fn abs(input: i32) -> i32;
}

fn add_to_count(inc: u32){
    unsafe{
        COUNTER += inc;
    }
}
