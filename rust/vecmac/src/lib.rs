#![allow(unused)]

macro_rules! alias {
    ($arg1: ident,  $arg2: ty) => {
        type $arg1 = $arg2;
    };
}

alias!(Shiva32, i32);
type Shiv = i32;

fn main() {
    let x: Shiva32 = 10;
    let x: Shiv = 10;
}
