#[cfg(all(any(feature = "bytes-buf", feature = "bytes-buf-tokio2"), not(nightly)))]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fails/*.rs");
    t.compile_fail("tests/proc-fails/*.rs");
}

