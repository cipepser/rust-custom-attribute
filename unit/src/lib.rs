#[test]
fn check() {
    let x = 1;
    assert_eq!(x, 1);
}

#[test]
fn fail() {
    let x = 1;
    assert_eq!(x, 2);
}
