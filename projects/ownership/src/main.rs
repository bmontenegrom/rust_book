fn main() {
    let mut s1 = String::from("hello");
    println!("s1 before change: {s1}");
    
    change(&mut s1);
    println!("s1 after chamge: {s1}");

}

/* fn calculate_lenght(s: &String) -> usize {
    s.len()
} */

fn change(some_string : &mut String){
    some_string.push_str(", world");
}