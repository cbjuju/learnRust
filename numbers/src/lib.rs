pub fn print() {
    let numbers = vec![1, 2, 3, 4, 5];
    for n in &numbers {
        println!("{:?}", n);
    }
    println!("The length of the vector is {:?}.", numbers.len());
}
