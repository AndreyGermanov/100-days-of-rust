fn tic_tac_toe(f:[[&str;3];3]) -> String {
    let players = ["X","O"];
    for i in 0..players.len() {
        let s = players[i];
        if (f[0][0] == s && f[0][1] == s && f[0][2] == s) ||
            (f[1][0] == s && f[1][1] == s && f[1][2] == s) ||
            (f[2][0] == s && f[2][1] == s && f[2][2] == s) ||
            (f[0][0] == s && f[1][0] == s && f[2][0] == s) ||
            (f[0][1] == s && f[1][1] == s && f[2][1] == s) ||
            (f[0][2] == s && f[1][2] == s && f[2][2] == s) ||
            (f[0][0] == s && f[1][1] == s && f[2][2] == s) ||
            (f[0][2] == s && f[1][1] == s && f[2][0] == s) {
            return format!("Player {} wins", i+1);
        }
    }
    return "It's a Tie".to_string();
}

#[test]
fn test_tic_tac_toe() {
    assert_eq!(tic_tac_toe([
        ["X", "O", "O"],
        ["O", "X", "O"],
        ["O", "#", "X"]
    ]),"Player 1 wins");
    assert_eq!(tic_tac_toe([
        ["X", "O", "O"],
        ["O", "X", "O"],
        ["X", "#", "O"]
    ]),"Player 2 wins");
    assert_eq!(tic_tac_toe([
        ["X", "X", "O"],
        ["O", "X", "O"],
        ["X", "O", "#"]
    ]), "It's a Tie");
}