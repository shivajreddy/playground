use rand::prelude::*;

/* write these 2 lines in main.rs to use this
mod my_rand;
use my_rand::rand;
*/

pub fn rand() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..10)
}
