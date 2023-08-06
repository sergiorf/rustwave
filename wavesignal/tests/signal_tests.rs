// tests/signal_tests.rs

use wavesignal::Signal;

#[test]
fn test_signal_length() {
    let signal = Signal::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(signal.len(), 5);
}

#[test]
fn test_get_sample() {
    let signal = Signal::new(vec![1.0, 2.0, 3.0]);
    assert_eq!(signal.get_sample(1), Some(2.0));
    assert_eq!(signal.get_sample(10), None);
}