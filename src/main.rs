use algorithms::vec_sort;
fn main() {
    println!("Hello, world!");
    let mut vector: Vec<i64> = vec![];

    for number in (1..12).rev() {
        vector.push(number);
    }

    let sorted_vector: Vec<i64> = vec_sort(vector);

    println!("{:?}", sorted_vector);
}
