fn main() {
    let mut opt1 = Some(42);
    let value = opt1.take();

    assert_eq!(opt1, None);
    assert_eq!(value, Some(42));

    let mut opt2: Option<i32> = None;
    let value = opt2.take();

    assert_eq!(opt2, None);
    assert_eq!(value, None);
}
