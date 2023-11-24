fn main() {
    let mut vector = vec![1, 2, 6, 4, 5, 3, 3, 4, 10, 10 ];
    println!("la mediana es: {}", mediana(&mut vector));
    println!("la moda es: {}", moda(&vector));
}


fn mediana( vector : &mut Vec<i32>) -> f64 {
    vector.sort();
    if vector.len() % 2 == 1 {
        vector[vector.len()/2] as f64
    } else {
        let menor = vector[(vector.len()/2) - 1] as f64;
        let mayor = vector[vector.len()/2] as f64;
        (menor + mayor)/2.0
    }
}

fn moda(vector : &Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for i in vector {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    let mut mode: i32 = -1;
    for (key, value) in map {
        if value >= max {
            mode = *key;
            max = value;
        }
    }
    mode
}