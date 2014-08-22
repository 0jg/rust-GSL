//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

extern crate rgsl;

use rgsl::RngType;

fn compare_func(a: &f64, b: &f64) -> i32 {
    if a > b {
        1
    } else if a < b {
        -1
    } else {
        0
    }
}

fn main() {
    rgsl::RngType::env_setup();

    let t : RngType = rgsl::rng::default();
    let r = rgsl::Rng::new(&t).unwrap();
    let k = 5;
    let n = 100000;
    let mut x : [f64, ..100000] = [0f64, ..100000];
    let mut small : [f64, ..5] = [0f64, ..5];
    let mut p : [u64, ..5] = [0u64, ..5];

    for tmp in range(0, n) {
        x[tmp] = r.uniform();
    }

    rgsl::sort::select::sort_smallest(small, k, x, 1);
    println!("{} smallest values from {}", k, n);
    for tmp in range(0, k as uint) {
        println!("{}: {}", k, small[tmp]);
    }

    rgsl::sort::select::sort_largest(small, k, x, 1);
    println!("\n{} largest values from {}", k, n);
    for tmp in range(0, k as uint) {
        println!("{}: {}", k, small[tmp]);
    }

    small.swap(2, 3);
    rgsl::sort::objects::heapsort_index(p, small, compare_func);
    println!("\nheapsort_index :", );
    for tmp in range(0, k as uint) {
        println!("{}: {}", k, p[tmp]);
    }

    rgsl::sort::objects::heapsort(small, compare_func);
    println!("\nheapsort :", );
    for tmp in range(0, k as uint) {
        println!("{}: {}", k, small[tmp]);
    }
}