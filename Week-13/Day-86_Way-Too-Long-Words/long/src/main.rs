fn shorten(word: &str) -> String {
    if word.len() < 10 { word.to_string() } else {
        format!("{}{}{}", word.chars().nth(0).unwrap(),word.len()-2,word.chars().last().unwrap())
    }
}

#[test]
fn test_shorten() {
    assert_eq!(shorten("word"),"word");
    assert_eq!(shorten("localization"),"l10n");
    assert_eq!(shorten("internationalization"),"i18n");
    assert_eq!(shorten("pneumonoultramicroscopicsilicovolcanoconiosis"),"p43s");
}