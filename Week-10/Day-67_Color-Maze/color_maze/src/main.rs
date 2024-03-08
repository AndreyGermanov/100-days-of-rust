fn solve_maze(seq: &[char],maze: &[&[char]]) -> Option<Vec<(usize,usize)>> {
    let mut result = vec![];
    let mut current_color_index = 0;
    let mut y = maze.len() -1;
    loop {
        let mut found = false;
        for x in 0..maze[y].len() {
            if maze[y][x] == seq[current_color_index] {
                result.push((x,y));
                current_color_index = if current_color_index == seq.len() - 1 {0} else { current_color_index + 1};
                found = true;
                if y+1 < maze.len() && maze[y+1][x] == seq[current_color_index] {
                    break
                }
            } else if x == maze[y].len()-1 && !found {
                return None
            }
        }
        if y==0 {
            break Some(result)
        }
        y -= 1;
    }
}

#[test]
fn test_solve_maze() {
    let mut result = solve_maze(&['R','W'],&[
        &['R', 'O', 'G'],
        &['W', 'R', 'W'],
        &['G', 'O', 'R'],
        &['O', 'R', 'W'],
    ]).unwrap();
    assert!(result == &[(1, 4), (1, 3), (1, 2), (3, 1), (1, 0)] ||
        result == &[(1, 3), (2, 3), (2, 2), (0, 1), (1, 1), (2, 1), (0, 0)]
    );

    assert_eq!(solve_maze(&['R','W'],&[
        &['R', 'O', 'G'],
        &['W', 'R', 'W'],
        &['G', 'O', 'O'],
        &['O', 'R', 'W'],
    ]),None);

    result = solve_maze(&['O','G'],&[
        &['C','O','R','O','Y'],
        &['O','R','V','G','R'],
        &['G','O','G','O','G'],
        &['Y','G','B','Y','G'],
        &['R','O','R','B','R'],
    ]).unwrap();
    assert!(result == &[(1, 4),(1, 3),(1, 2),(2, 2),(3, 2),(3, 1),(3, 0)] ||
        result == [(1, 4), (1, 3), (1, 2), (3, 1), (1, 0)]);

    result =  solve_maze(&['O','G'], &[
        &['B','O','R','O','Y'],
        &['O','R','B','G','R'],
        &['B','O','G','O','Y'],
        &['Y','G','B','Y','G'],
        &['R','O','R','B','R'],
    ]).unwrap();
    assert!(result == &[(1, 4),(1, 3),(1, 2),(2, 2),(3, 2),(3, 1),(3, 0)] ||
        result == &[(1, 4), (1, 3), (1, 2), (3, 1), (1, 0)]);

    assert_eq!(solve_maze(&['O','G'], &[
        &['G','O','R','O','Y'],
        &['O','R','B','C','R'],
        &['G','O','G','O','G'],
        &['Y','G','B','Y','G'],
        &['R','O','R','B','R']
    ]), None);
}
