#[cfg(test)]
mod test_fail {
    const BINARIES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/binaries/ROPemporium");

    #[test]
    fn test_x86_fail() {
        let targets: Vec<String> = vec![
            format!("{}/{}", BINARIES_DIR, "x86/1-split/split32"),
            format!("{}/{}", BINARIES_DIR, "x86/2-callme/callme32"),
            format!("{}/{}", BINARIES_DIR, "x86/2-callme/libcallme32.so"),
            format!("{}/{}", BINARIES_DIR, "x86/3-write4/write432"),
            format!("{}/{}", BINARIES_DIR, "x86/3-write4/libwrite432.so"),
            format!("{}/{}", BINARIES_DIR, "x86/4-badchars/badchars32"),
            format!("{}/{}", BINARIES_DIR, "x86/4-badchars/libbadchars32.so"),
        ];
        for target in targets {
            println!("Processing {}", target);
            let bv = binja_load_uaf::binja_load(target);
            assert!(bv.is_ok());
        }
    }
}
