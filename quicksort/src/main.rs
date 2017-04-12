extern crate quicksort;

fn main() {
    let mut a = [9,8,7,6,5,4,3,2,1];
    println!("{:?}", a);
    quicksort::quicksort(&mut a);
    println!("{:?}", a);
}
