fn main() {
    // # Vec数据类型
    // Specify the capacity
    let mut num_vec = Vec::with_capacity(8); // Give it capacity 8
    num_vec.push('a'); // add one character
    println!("{}", num_vec.capacity()); // prints 8
    num_vec.push('a'); // add one more
    println!("{}", num_vec.capacity()); // prints 8
    num_vec.push('a'); // add one more
    println!("{}", num_vec.capacity()); // prints 8.
    num_vec.push('a'); // add one more
    num_vec.push('a'); // add one more // Now we have 5 elements
    println!("{}", num_vec.capacity()); // Still 8
    // Create the `Vec` using `into` method
    let my_vec: Vec<u8> = [1, 2, 3].into();
    let my_vec2: Vec<_> = [9, 0, 10].into();
    println!("{:?} {:?}", my_vec, my_vec2);
    // Display array
    let my_arr = [1, 2, 3];
    let my_arr1 = [9, 0, 10];
    display_array(&my_arr);
    display_array(&my_arr1);
    display_array1(my_arr);
    display_array1(my_arr1);
}

fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    println!("Array: {:?}", arr);
}

fn display_array1<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("Array: {:?}", arr)
}
