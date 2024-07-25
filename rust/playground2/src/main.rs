fn main() {
    let v = vec![Some(42), Some(43), None, Some(44)];
    // let v = vec![Some(42), Some(43), Some(44)];

    let mut idx = 0;

    while idx < v.len() {
        if let Some(val) = v[idx] {
            println!("{}", val);
            idx += 1;
        } else {
            println!("None");
            idx += 1;
        }
    }
}
