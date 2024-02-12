fn min_pages(n:u32, p:u32) -> u32 {
    return (p / 2).min(if p == n - 1 && n % 2 == 0 { 1 } else { (n - p) / 2})
}

#[test]
fn test_min_pages() {
    assert_eq!(min_pages(2, 1), 0);
    assert_eq!(min_pages(5, 3), 1);
    assert_eq!(min_pages(6, 2), 1);
    assert_eq!(min_pages(5, 4), 0);
    assert_eq!(min_pages(4, 4), 0);
    assert_eq!(min_pages(1, 1), 0);
    assert_eq!(min_pages(7, 3), 1);
    assert_eq!(min_pages(6, 3), 1);
    assert_eq!(min_pages(6, 4), 1);
    assert_eq!(min_pages(37455, 29835), 3810);
    assert_eq!(min_pages(95073, 17440), 8720);
    assert_eq!(min_pages(73201, 57075), 8063);
    assert_eq!(min_pages(59867, 35733), 12067);
    assert_eq!(min_pages(15603, 6957), 3478);
    assert_eq!(min_pages(15600, 1560), 780);
    assert_eq!(min_pages(5809, 2668), 1334);
    assert_eq!(min_pages(86619, 28906), 14453);
    assert_eq!(min_pages(60113, 8589), 4294);
    assert_eq!(min_pages(70809, 46090), 12359);
    assert_eq!(min_pages(2059, 117), 58);
    assert_eq!(min_pages(96993, 70030), 13481);
    assert_eq!(min_pages(83246, 78132), 2557);
    assert_eq!(min_pages(18183, 18042), 70);
    assert_eq!(min_pages(6, 5), 1);
}
