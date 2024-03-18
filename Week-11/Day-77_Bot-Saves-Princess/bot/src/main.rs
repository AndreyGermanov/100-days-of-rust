use std::iter;

fn find_princess<'a>(r:usize, c:usize, grid: &'a[&'a[char]]) -> Vec<&'a str> {
    let (p_row,p_col) = grid.iter().enumerate().fold((0,0),
    |(p_row,p_col),(index,row)| {
            match row.iter().position(|item| *item == 'p') {
                Some(col) => (index, col),
                _ => (p_row , p_col)
            }
        }
    );
    let (hor,vert) = (p_col as i8 - c as i8,p_row as i8 - r as i8);
    iter::repeat(if hor>0 {"RIGHT"} else {"LEFT"}).take(i8::abs(hor) as usize)
        .chain(
        iter::repeat(if vert>0 {"DOWN"} else {"UP"}).take(i8::abs(vert) as usize)
        ).collect()
}

#[test]
fn test_find_princess() {
    assert_eq!(find_princess(2,3,&[
        &['-', '-', '-', '-', '-'],
        &['-', '-', '-', '-', '-'],
        &['p', '-', '-', 'm', '-'],
        &['-', '-', '-', '-', '-'],
        &['-', '-', '-', '-', '-']
    ]),vec!["LEFT","LEFT","LEFT"]);
    assert_eq!(find_princess(4,4,&[
        &['-','-','-','-','-'],
        &['-','-','-','-','-'],
        &['p','-','-','-','-'],
        &['-','-','-','-','-'],
        &['-','-','-','-','m']
    ]),vec!["LEFT","LEFT","LEFT","LEFT","UP","UP"]);
    assert_eq!(find_princess(2,0,&[
        &['-','-','-','-','-'],
        &['-','-','-','-','-'],
        &['m','-','-','-','-'],
        &['-','-','-','-','-'],
        &['-','-','-','-','p']
    ]),vec!["RIGHT","RIGHT","RIGHT","RIGHT","DOWN","DOWN"]);
}
