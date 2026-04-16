pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max: f64 = self.value as f64/ self.max as f64;
        
        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning you have used up 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Warning you have used up 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("You have used up you quota");
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_message: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {sent_message: vec![]}
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_message.push(String::from(message));
        }
    }
    
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger: MockMessenger = MockMessenger::new();
        let mut limit_tracker: LimitTracker<'_, MockMessenger> = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_message.len(), 1);
    }
}