fn split_to_ip(mut ips: String) -> Vec<String> {
    if ips.len() < 4 || ips.len() > 12 {
        return vec![];
    }
    let mut res = vec![];
    backtrack(&mut ips, 1, 0, &mut res);
    res
}

fn backtrack(ips: &mut String, start: usize, parts: usize, res: &mut Vec<String>) {
    if parts > 3 {
        return;
    }
    if parts == 3 && is_valid_ip(ips) {
        res.push(ips.clone());
        return;
    }
    for index in start..ips.len() {
        let mut ips_copy = ips.clone();
        ips_copy.insert(index, '.');
        backtrack(&mut ips_copy, index + 2, parts + 1, res);
    }
}

fn is_valid_ip(ip_string: &mut str) -> bool {
    let ip = ip_string.split('.').filter(|&ip_part| !ip_part.is_empty()).collect::<Vec<_>>();
    ip.len() == 4
        && ip.into_iter().all(|ip_part| {
        if ip_part.len() > 1 && ip_part.chars().take(1).collect::<Vec<_>>()[0] == '0' {
            return false;
        }
        ip_part.parse::<u32>().unwrap() <= 255
    })
}

#[test]
fn test_split_to_ip() {
    assert_eq!(split_to_ip("25525511135".to_string()), vec!["255.255.11.135","255.255.111.35"]);
    assert_eq!(split_to_ip("0000".to_string()), vec!["0.0.0.0"]);
    assert_eq!(split_to_ip("1111".to_string()), vec!["1.1.1.1"]);
    assert_eq!(split_to_ip("010010".to_string()), vec!["0.10.0.10","0.100.1.0"]);
    assert_eq!(split_to_ip("101023".to_string()), vec!["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]);
}
