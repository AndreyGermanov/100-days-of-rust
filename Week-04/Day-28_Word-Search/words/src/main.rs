mod trie;

use std::collections::HashSet;
use trie::Trie;

pub fn match_words(board: &[&[char]], words: Vec<String>) -> Vec<String> {
    let width = board[0].len();
    let height = board.len();

    let letters: HashSet<_> = board.iter().flat_map(|row| row.iter().map(|c| *c as u8)).collect();
    let word_count = words.len();
    let mut words = {
        let max_len = width * height;
        let mut trie = Trie::new();
        for w in words {
            if w.len() > max_len || !w.as_bytes().iter().all(|c| letters.contains(&c)) {
                continue;
            }
            trie.insert(w.as_bytes());
        }
        trie
    };

    let mut results = Vec::new();
    if words.is_empty() {
        return results;
    }

    let mut board: Vec<Vec<u8>> = board.into_iter().map(|row| row.into_iter().map(|c| *c as u8).collect()).collect();
    'both: for x in 0..width {
        for y in 0..height {
            let cell = (x as i8, y as i8);
            solve(&mut *board, cell, &mut words, &mut String::new(), &mut results);
            if results.len() == word_count {
                break 'both;
            }
        }
    }

    results.sort();
    results
}

fn solve(board: &mut [Vec<u8>], cell: (i8, i8), words: &mut Trie, word: &mut String, results: &mut Vec<String>) {
    let (letter, trie) = {
        let letter = &mut board[cell.1 as usize][cell.0 as usize];
        let trie = match words.children.get_mut(letter) {
            Some(trie) => trie,
            None => return,
        };
        (std::mem::take(letter), trie)
    };

    word.push(letter as char);
    if trie.is_end {
        // Remove this word from the trie
        trie.is_end = false;
        results.push(word.clone());
    }

    let width = board[0].len().try_into().unwrap();
    let height = board.len().try_into().unwrap();

    let mut prune_child = trie.children.is_empty();
    if !prune_child {
        let neighbors = [(cell.0 - 1, cell.1), (cell.0 + 1, cell.1), (cell.0, cell.1 - 1), (cell.0, cell.1 + 1)];
        for (x, y) in neighbors {
            if x < 0 || x >= width || y < 0 || y >= height {
                continue;
            }
            if board[y as usize][x as usize] == 0 {
                continue;
            }
            solve(&mut *board, (x, y), trie, &mut *word, &mut *results);
            if trie.children.is_empty() {
                prune_child = true;
                break;
            }
        }
    }

    if prune_child {
        words.children.remove(&letter);
    }

    word.pop();
    board[cell.1 as usize][cell.0 as usize] = letter;
}

#[test]
fn test_match_words() {
    assert_eq!(match_words(&[
        &['o','a','a','n'],
        &['e','t','a','e'],
        &['i','h','k','r'],
        &['i','f','l','v']
    ], vec!["oath", "pea", "eat", "rain"].iter().map(|s| s.to_string()).collect()),vec!["eat","oath"])
}
