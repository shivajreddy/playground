#![allow(unused)]

<<<<<<< HEAD
pub trait Messenger {
    fn send(&self, message: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    max: usize,
    value: usize,
    messenger: &'a T
}


impl<'a, T: Messenger> LimitTracker<'a, T> {
    fn new(messenger: &'a T) -> LimitTracker<'a, T> {
        LimitTracker {
            max: 100,
            value: 0,
            messenger
        }
    }
    
    fn set_value(&mut self, value: usize) {
        self.value = value;
        let mut percent = self.value as f64 / self.max as f64;
        if percent > 1.0 {
            self.messenger.send("Over 1.0");
        } else if percent > 0.9 {
            self.messenger.send("Over 0.9");
        } else if percent > 0.7 {
            self.messenger.send("Over 0.7");
        }
    }
    
=======
trait Messenger {
    fn send(&self, message: String);
    // fn send(&self, message: &str);
>>>>>>> 057e14deb78e6a13fe8ec7343081afb6ef788559
}



<<<<<<< HEAD
=======
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



>>>>>>> 057e14deb78e6a13fe8ec7343081afb6ef788559
#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::cell::RefCell;
    use super::*;
<<<<<<< HEAD
    
    struct MockMessenger {
        // outbox: Vec<String>
        outbox: Rc<RefCell<Vec<String>>>
    }
    
    impl MockMessenger {
        fn new() -> Self {
            // MockMessenger {outbox: vec![]}
            MockMessenger {outbox: Rc::new(RefCell::new(vec![]))}
        }
    }
    
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // self.outbox.push(String::from(message));
            let rc1 = Rc::clone(&self.outbox);
            let rc2 = Rc::clone(&self.outbox);
            rc1.borrow_mut().push(String::from(message));
            // rc2.borrow_mut().push(String::from(message));
        }
    }
    
    #[test]
    fn test_message(){
        let mut mock_messenger = MockMessenger::new();
        
        let mut tracker = LimitTracker::new(&mock_messenger);
        tracker.set_value(90);
        assert_eq!(mock_messenger.outbox.borrow().len(), 1);
    }
=======
>>>>>>> 057e14deb78e6a13fe8ec7343081afb6ef788559
}

