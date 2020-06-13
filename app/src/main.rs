use utils::{test_case, run_inventory_tests};

#[test_case]
fn check() {
    let x = 1;
    assert_eq!(x, 1);
}

#[test_case]
fn fail() {
    let x = 1;
    assert_eq!(x, 2);
}

fn main() {
    run_inventory_tests!();
}