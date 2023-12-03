fn main() {
    let v1 = vec![1, 2, 3];
    let vi_iter = v1.iter();
    for val in vi_iter {
        println!("Got: {val}");
    }

    let v2 = vec![1, 2, 3];
    let v2_iter = v2.iter();
    let total: i32 = v2_iter.sum();
    println!("total : {total}");
    //map takes a closure to call on each item
    let v3: Vec<_> = v2.iter().map(|x| x +1).collect();
    println!("v3 : {:?}", v3);
}
