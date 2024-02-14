fn next_edge(edge1:u32, edge2:u32) -> u32 { edge1 + edge2 - 1 }

#[test]
fn test_next_edge() {
    assert_eq!(next_edge(8, 10), 17);
    assert_eq!(next_edge(5, 7), 11);
    assert_eq!(next_edge(9, 2), 10);
}
