fn compress(source: &str, target: &str) {
    let bytes = std::fs::read(source).unwrap();
    let mut encoding;

    if bytes.first().is_none() {
        return;
    } else {
        encoding = vec![*bytes.first().unwrap()];
    }
    let mut occurrences = 1;
    for byte in bytes.iter().skip(1) {
        if byte == encoding.last().unwrap() && occurrences < 255 {
            occurrences += 1;
        } else {
            encoding.extend(&[occurrences, *byte]);
            occurrences = 1;
        }
    }
    encoding.push(occurrences);
    std::fs::write(target, encoding).expect("Could not save the file");
}

fn decompress(source: &str, target: &str) {
    let bytes:Vec<_> = std::fs::read(source).unwrap();
    let mut decoding = Vec::<u8>::new();
    for (i, byte) in bytes.iter().enumerate() {
        if i % 2 != 0 {
            continue;
        }
        for _j in 0..bytes[i+1] {
            decoding.push(*byte)
        }
    }
    std::fs::write(target, decoding).expect("Could not save the file");
}

#[test]
fn test_compress_decompress() {
    compress("src/main.rs", "main.lzw");
    decompress("main.lzw", "output.txt");
    assert_eq!(std::fs::read_to_string("src/main.rs").unwrap(),
               std::fs::read_to_string("output.txt").unwrap()
    );
}
