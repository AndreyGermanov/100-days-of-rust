use std::collections::HashMap;
fn is_anagram(s: &str, t: &str) -> bool {
    let mut chars: HashMap<char, i32> = HashMap::new();
    s.to_lowercase().chars().filter(|ch| (*ch as u32) > 96 && (*ch as u32) < 123).for_each(|ch| {
        if chars.contains_key(&ch) {
            *chars.get_mut(&ch).unwrap() += 1;
        } else {
            chars.insert(ch, 1);
        }
    });
    t.to_lowercase().chars().for_each(|ch| {
        if chars.contains_key(&ch) {
            *chars.get_mut( &ch).unwrap() -= 1;
        }
    });
    for (ch, count) in chars {
        if count != 0 {
            return false;
        }
    }
    return true;
}

#[test]
fn test_is_anagram() {
    assert_eq!(is_anagram("anagram","nagaram"), true);
    assert_eq!(is_anagram("car","rat"), false);
    assert_eq!(is_anagram("New York Times","monkeys write"), true);
    assert_eq!(is_anagram("Church of Scientology","rich-chosen goofy cult"), true);
    assert_eq!(is_anagram("McDonald's restaurants","Uncle Sam's standard rot"), true);
    assert_eq!(is_anagram("coronavirus","carnivorous"), true);
    assert_eq!(is_anagram("She Sells Sanctuary","Santa; shy, less cruel"), true);
    assert_eq!(is_anagram("She Sells Sanctuary","Satan; cruel, less shy"), true);
    assert_eq!(is_anagram("evil","vile"), true);
    assert_eq!(is_anagram("a gentleman","elegant man"), true);
    assert_eq!(is_anagram("restful","fluster"), true);
    assert_eq!(is_anagram("cheater","teacher"), true);
    assert_eq!(is_anagram("funeral","real fun"), true);
    assert_eq!(is_anagram("forty five","over fifty"), true);
    assert_eq!(is_anagram("Santa","Satan"), true);
    assert_eq!(is_anagram("William Shakespeare","I am a weakish speller"), true);
    assert_eq!(is_anagram("Madam Curie","Radium came"), true);
    assert_eq!(is_anagram("George Bush","He bugs Gore"), true);
    assert_eq!(is_anagram("Tom Marvolo Riddle","I am Lord Voldemort"), true);
}
