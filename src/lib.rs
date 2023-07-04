pub fn vec_sort(vector: Vec<i64>) -> Vec<i64> {
    let sorted_vector: Vec<i64> = sorting::merge_sort(vector);
    println!("{:?}", sorted_vector);
    sorted_vector
}

mod sorting {
    pub fn merge_sort(vector: Vec<i64>) -> Vec<i64> {
        let half: usize = vector.len() / 2;

        //check vector length. If greater than 2, carry on.
        if vector.len() < 2 {
            return vector;
        }
        //Recursive! split and go back to the beginning
        let left: Vec<i64> = merge_sort(vector[..half].to_vec());
        let right: Vec<i64> = merge_sort(vector[half..].to_vec());

        merge(left, right)
    }

    fn merge(mut left: Vec<i64>, mut right: Vec<i64>) -> Vec<i64> {
        let mut result: Vec<_> = Vec::new();

        //Compare variables and see which side is bigger
        while !left.is_empty() && !right.is_empty() {
            if left[0] <= right[0] {
                result.push(left.remove(0));
            } else {
                result.push(right.remove(0));
            }
        }

        //Take care of remaining elements
        while !left.is_empty() {
            result.push(left.remove(0));
        }
        while !right.is_empty() {
            result.push(right.remove(0));
        }

        result
    }
}
