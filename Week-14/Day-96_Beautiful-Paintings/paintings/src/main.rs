fn paintings(input: Vec<usize>) -> usize {
    let mut paints = input;
    paints.sort();
    let mut last = paints[0];
    let mut result = 0;
    for index in 1..paints.len() {
        let mut found = paints[index] > last;
        if !found {
            for index2 in index..paints.len() {
                if paints[index2] > last {
                    last = paints[index2];
                    found = true;
                    result += 1;
                    break;
                }
            }
        }
        if !found { last = paints[index]} else { result += 1}
    }
    result
}

#[test]
fn test_paintings() {
    assert_eq!(paintings(vec![20,30,10,50,40]),4);
    assert_eq!(paintings(vec![200,100,100,200]),2);
}


