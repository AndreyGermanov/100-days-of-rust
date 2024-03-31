use std::collections::HashMap;
fn do_tasks(n: usize, m:usize, text: &str) -> String {
    let lines:Vec<&str> = text.split("\n").collect();
    let mut ips:HashMap<String,String> = HashMap::new();
    (0..lines.len()).fold(vec![],|mut result,index| {
        let line = lines[index].split(" ").collect::<Vec<&str>>();
        if index < n {
            ips.insert(line[1].to_string(),line[0].to_string());
            return result
        }
        result.push(format!("{} {} #{}",line[0],line[1],ips[&line[1].replace(";","")]));
        return result
    }).join("\n")
}

#[test]
fn test_do_tasks() {
    assert_eq!(do_tasks(2,2, "main 192.168.0.2
replica 192.168.0.1
block 192.168.0.1;
proxy 192.168.0.2;"), "block 192.168.0.1; #replica
proxy 192.168.0.2; #main");
    assert_eq!(do_tasks(3, 5,"google 8.8.8.8
codeforces 212.193.33.27
server 138.197.64.57
redirect 138.197.64.57;
block 8.8.8.8;
cf 212.193.33.27;
unblock 8.8.8.8;
check 138.197.64.57;"), "redirect 138.197.64.57; #server
block 8.8.8.8; #google
cf 212.193.33.27; #codeforces
unblock 8.8.8.8; #google
check 138.197.64.57; #server")
}

