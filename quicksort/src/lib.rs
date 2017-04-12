extern crate partition;

use partition::partition;

pub fn quicksort<T: PartialOrd + Clone>(a: &mut[T]) {
    if a.len() <= 1 {
        return
    }

    let pivot = a[0].clone();
    let (mut smaller, mut greater_equal) = partition(&mut a[..], |x| x < &pivot);
    let (_, mut greater) = partition(&mut greater_equal, |x| x == &pivot);

    quicksort(&mut smaller);
    quicksort(&mut greater);
}
