mod common;

#[test]
fn test_add() {
    assert_eq!(integration_testing::add(3, 2), 5);
    assert_eq!(common::CASES.len(), 2);
    for (left, right, expected) in common::CASES {
        assert_eq!(integration_testing::add(left, right), expected);
    }
}
