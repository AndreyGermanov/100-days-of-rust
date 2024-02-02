use itertools::Itertools;

fn climb_leaderboard(ranked: &[u8], player: &[u8]) -> Vec<usize> {
    let mut ranked: Vec<_> = ranked.into_iter().unique().collect();
    ranked.sort();ranked.reverse();
    let mut result = vec![];
    let mut ranked_len = ranked.len();
    for score in player {
        while ranked_len > 0 && score >= ranked[ranked_len-1] {
            ranked_len -= 1;
        }
        result.push(ranked_len+1)
    }
    result
}

#[test]
fn test_climb_leaderboard() {
    assert_eq!(climb_leaderboard(&[100, 90, 90, 80], &[70, 80, 105]), vec![4,3,1]);
    assert_eq!(climb_leaderboard(&[100, 90, 90, 80], &[106, 107, 105]), vec![1, 1, 1]);
}
