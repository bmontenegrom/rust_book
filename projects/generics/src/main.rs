fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("the largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("the largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = larges_generic(&char_list);
    println!("the largest element is {result}");
}




fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list{
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn larges_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}