macro_rules! sha2_test {
    ($test_name:ident, $kats_fn:path, $hash_fn:path) => {
        #[test]
        #[allow(non_snake_case)]
        fn $test_name() {
            let tv = $kats_fn();
            let test_cnt = tv.tests.len();
            assert!(test_cnt > 0, "Empty test vector file");
            for (i, test) in tv.tests.iter().enumerate() {
                let digest = $hash_fn(&test.msg[0..test.msg_length / 8]);
                assert_eq!(&digest[..], &test.digest[..], "test {i}: digest mismatch");
            }
            eprintln!("Ran {test_cnt} tests for {}", stringify!($test_name));
        }
    };
}

sha2_test!(
    SHA224ShortMsg,
    test_foo_bar_kats::sha2::sha224_short,
    test_foo_bar_sha2::sha224
);
sha2_test!(
    SHA224LongMsg,
    test_foo_bar_kats::sha2::sha224_long,
    test_foo_bar_sha2::sha224
);

sha2_test!(
    SHA256ShortMsg,
    test_foo_bar_kats::sha2::sha256_short,
    test_foo_bar_sha2::sha256
);
sha2_test!(
    SHA256LongMsg,
    test_foo_bar_kats::sha2::sha256_long,
    test_foo_bar_sha2::sha256
);

sha2_test!(
    SHA384ShortMsg,
    test_foo_bar_kats::sha2::sha384_short,
    test_foo_bar_sha2::sha384
);
sha2_test!(
    SHA384LongMsg,
    test_foo_bar_kats::sha2::sha384_long,
    test_foo_bar_sha2::sha384
);

sha2_test!(
    SHA512ShortMsg,
    test_foo_bar_kats::sha2::sha512_short,
    test_foo_bar_sha2::sha512
);
sha2_test!(
    SHA512LongMsg,
    test_foo_bar_kats::sha2::sha512_long,
    test_foo_bar_sha2::sha512
);
