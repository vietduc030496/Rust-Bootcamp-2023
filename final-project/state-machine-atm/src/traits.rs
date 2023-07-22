use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

/// A state machine - Generic over the transition type
pub trait StateMachine {
    /// The states that can be occupied by this machine
    type State;

    /// The transitions that can be made between states
    type Transition;

    /// Calculate the resulting state when this state undergoes the given transition
    fn next_state(starting_state: &Self::State, t: &Self::Transition) -> Self::State;
}

// Simple helper to do some hashing.
pub fn hash<T: fmt::Display>(t: &[T]) -> u64 {
    let mut a: String = String::new();
    for i in t {
        match i.to_string().as_str() {
            "One" =>  a.push('1'),
            "Two" => a.push('2'),
            "Three" => a.push('3'),
            "Four" => a.push('4'),
            _ => (),
        }
    }
    a.parse::<u64>().unwrap()
}
// fn hash<T: Hash>(t: &T) -> u64 {
//     let mut s = DefaultHasher::new();
//     t.hash(&mut s);
//     s.finish()
// }
// Test for hash function
#[test]
fn test_hash_enum_vec() {
    enum KeyTest {
        One,
        Two,
        Three,
        Four,
    }

    impl fmt::Display for KeyTest {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                KeyTest::One => write!(f, "One"),
                KeyTest::Two => write!(f, "Two"),
                KeyTest::Three => write!(f, "Three"),
                KeyTest::Four => write!(f, "Four"),
            }
        }
    }

    let input: Vec<KeyTest> = vec![KeyTest::One, KeyTest::Two, KeyTest::Three, KeyTest::Four];

    let hash1 = hash(&input);
    let hash2 = hash(&input);

    assert_eq!(hash1, hash2);
}