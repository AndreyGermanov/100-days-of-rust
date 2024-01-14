#[derive(Eq, PartialEq)]
enum LandscapeType {
    MOUNTAIN,
    VALLEY,
    NEITHER
}

impl LandscapeType {
    fn to_string(&self) -> &str {
        match self {
            LandscapeType::MOUNTAIN => "mountain",
            LandscapeType::VALLEY => "valley",
            LandscapeType::NEITHER => "neither"
        }
    }
}

fn landscape_type(heights:Vec<i32>) -> LandscapeType {
    let (max_index, min_index) = get_max_min_idx(&heights);
    return if is_mountain(&heights, max_index) {
            LandscapeType::MOUNTAIN
        } else if is_valley(&heights, min_index) {
            LandscapeType::VALLEY
        } else {
            LandscapeType::NEITHER
        }
}

fn is_mountain(heights:&[i32], max_index:usize) -> bool {
    if max_index == 0 || max_index == heights.len() - 1 {
        return false;
    }
    let left = &mut heights[0..max_index].to_vec();
    left.sort();
    let right = &mut heights[(max_index+1) ..heights.len()].to_vec();
    right.sort_by(|a, b| b.cmp(a));
    left == &heights[0..max_index] && right == &heights[(max_index+1)..heights.len()]
}

fn is_valley(heights:&[i32], min_index:usize) -> bool {
    if min_index == 0 || min_index == heights.len() - 1 {
        return false
    }
    let left = &mut heights[0..min_index].to_vec();
    left.sort_by(|a, b| b.cmp(a));
    let right = &mut heights[(min_index+1) ..heights.len()].to_vec();
    right.sort();
    left == &heights[0..min_index] && right == &heights[(min_index+1) ..heights.len()]
}

fn get_max_min_idx(arr:&[i32]) -> (usize, usize) {
    let (mut max_index, mut min_index) = (0, 0);
    for index in 0..arr.len() {
        if arr[index] > arr[max_index] {
            max_index = index;
        }
        if arr[index] < arr[min_index] {
            min_index = index;
        }
    }
    return (max_index, min_index);
}

#[test]
fn test_landscape_type() {
    assert_eq!(landscape_type(vec![1, 3, 5, 4, 3, 2]).to_string(), "mountain");
    assert_eq!(landscape_type(vec![10, 9, 8, 7, 2, 3, 4, 5]).to_string(), "valley");
    assert_eq!(landscape_type(vec![-1, 0, -1]).to_string(), "mountain");
    assert_eq!(landscape_type(vec![-1, -1, 0, -1, -1]).to_string(), "mountain");
    assert_eq!(landscape_type(vec![10, 9, 8, 7, 2, 3, 4, 5]).to_string(), "valley");
    assert_eq!(landscape_type(vec![350, 100, 200, 400, 700]).to_string(), "valley");
    assert_eq!(landscape_type(vec![1, 2, 3, 2, 4, 1]).to_string(), "neither");
    assert_eq!(landscape_type(vec![5, 4, 3, 2, 1]).to_string(), "neither");
    assert_eq!(landscape_type(vec![0, -1, -1, 0, -1, -1]).to_string(), "neither");
    assert_eq!(landscape_type(vec![3, 4, 5, 4, 3]).to_string(), "mountain");
    assert_eq!(landscape_type(vec![9, 7, 3, 1, 2, 4]).to_string(), "valley");
    assert_eq!(landscape_type(vec![9, 8, 9]).to_string(), "valley");
    assert_eq!(landscape_type(vec![9, 8, 9, 8]).to_string(), "neither");
}
