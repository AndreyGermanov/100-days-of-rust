fn light_on(arr: &mut Vec<[u8; 2]>) -> u32 {
    arr.sort_by(|i1, i2| { i1.partial_cmp(i2).unwrap() });
    let mut i = 0;
    while i < arr.len() - 1 {
        if arr[i+1][0] > arr[i][0] && arr[i+1][0] < arr[i][1]{
            arr[i] = [arr[i][0].min(arr[i+1][0]),arr[i][1].max(arr[i+1][1])];
            arr.remove(i+1);
        } else {
            i += 1;
        }
    }
    arr.iter().fold(0, |sum,item| { sum + (item[1] - item[0]) as u32 })
}

fn light_on2(arr: &mut Vec<[u8; 2]>) -> u32 {
    let (mut u,mut n) = (0,0);
    arr.iter().for_each(|item| {
        n = (1 << item[1]) - (1 << item[0]);
        u |= n;
    });
    format!("{u:b}").chars().filter(|ch| *ch == '1').count() as u32
}

#[test]
fn test_light_on() {
    assert_eq!(light_on(&mut vec![[1,3],[2,3],[4,5]]),3);
    assert_eq!(light_on(&mut vec![[2,4],[3,6],[1,3],[6,8]]),7);
    assert_eq!(light_on(&mut vec![[6,8],[5,8],[8,9],[5,7],[4,7]]),5);

    assert_eq!(light_on2(&mut vec![[1,3],[2,3],[4,5]]),3);
    assert_eq!(light_on2(&mut vec![[2,4],[3,6],[1,3],[6,8]]),7);
    assert_eq!(light_on2(&mut vec![[6,8],[5,8],[8,9],[5,7],[4,7]]),5);
}
