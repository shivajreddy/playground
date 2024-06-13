#![allow(unused)]

trait Messenger {
    fn send(&self, message: String);
    // fn send(&self, message: &str);
}



struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,       // lives as long as this type instance
    max: usize,
    value: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { messenger, value: 0, max}
    }
    
    fn set_value(&mut self, value: usize) {
        self.value = value;
        let percent = self.value as f64 / self.max as f64;
        if percent > 1.0 {
            println!("OVER 100% : Banned");
        } else if percent > 0.9 {
            println!("OVER 90% : Soft Warning");
        } else if percent > 0.7 {
            println!("OVER 70% : Careful");
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
}
