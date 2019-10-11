// This is a pretty fragile test. it assumes we have 10 computers and 20
// visitors, and it only makes sure each visitor comes online and then leaves.
// This is mostly here as an example integration test, but feel free to
// come up with some more cases to check and submit a PR!

#[cfg(test)]
mod integration {
    use test_bin;

    #[test]
    fn output_contains_all_visitors() {
        let output = test_bin::get_test_bin("problem8")
            .output()
            .expect("ERROR: could not run problem 8. Has it been renamed?");

        let res = String::from_utf8_lossy(&output.stdout);

        for i in 1..=20 {
            let expected_msg1 = format!("Visitor {} is online", i);
            let expected_msg2 = format!("Visitor {} spent", i);
            assert!(res.contains(&expected_msg1));
            assert!(res.contains(&expected_msg2));
        }
    }
}
