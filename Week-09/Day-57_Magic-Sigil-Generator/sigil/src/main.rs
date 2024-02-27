use itertools::Itertools;

fn sigilize(input: &str) -> String {
    input.to_uppercase().chars().rev().collect::<Vec<_>>().iter().unique().filter(|ch| {
        !vec![' ','A','E','I','O','U'].contains(ch)
    }).map(|ch| *ch).collect::<Vec<_>>().iter().rev().collect()
}

#[test]
fn test_sigilize() {
    assert_eq!(sigilize("i am healthy"),"MLTHY");
    assert_eq!(sigilize("I FOUND MY SOULMATE"), "FNDYSLMT");
    assert_eq!(sigilize("I have a job I enjoy and it pays well"), "HVBJNDTPYSWL");
}
