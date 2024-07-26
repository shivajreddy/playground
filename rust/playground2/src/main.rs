fn main() {
    let mut v = vec![10, 20, 30];

    let x = &v[0];
    println!("{}", *x);

    {
        let y = &mut v;
    }

    let z = &v[0];
    println!("{}", *z);

    // println!("{}", *x);
}
