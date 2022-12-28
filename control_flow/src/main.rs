fn if_statement(x: i32) -> &'static str {
    if x < 10 {
        "less than 10"
    } else if x > 10 {
        "more than 10"
    } else {
        "equal to 10"
    }
}

fn loops(mut x: i32) {
    while x > 0 {
        println!("x value is {}!", x);
        x -= 1;
    }
}

// fn infinite_loop() {
//     loop {
//         println!("Infinite loop !");
//     }
// }

fn vec_iter_and_for_loop(v: &Vec<i32>) {
    println!("Values in vector:");
    for val in v.iter() {
        println!("{}", val);
    }
}

fn vec_iter_and_for_loop_with_id(v: &Vec<i32>) {
    for i in 0..v.len() {
        println!("Value at index {} is {}", i, v[i]);
    }
}

fn double_vector_values(v: &mut Vec<i32>) {
    for value in v.iter_mut() {
        *value *= 2;
    }
}

fn main() {
    println!("{}\n{}\n{}\n", if_statement(9), if_statement(10), if_statement(11));
    loops(10);
    // infinite_loop();
    let mut v: Vec<i32> = vec![1, 2, 3];
    vec_iter_and_for_loop(&v);
    double_vector_values(&mut v);
    vec_iter_and_for_loop_with_id(&v);
}
