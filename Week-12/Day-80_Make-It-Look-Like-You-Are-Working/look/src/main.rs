use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    loop { list_dir("/home"); }
}

fn list_dir(root: &str) {
    if let Ok(list) = fs::read_dir(root) {
        list.for_each(|dir| {
            if let Ok(dir) = dir {
                println!("{}/{}",root,dir.file_name().to_str().unwrap());
                thread::sleep(Duration::from_secs(1));
                if dir.path().is_dir() {
                    list_dir(format!("{}/{}",root,dir.file_name().to_str().unwrap()).as_str())
                }
            }
        })
    }
}