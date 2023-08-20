use addr;

#[test]
fn it_works3() {
    let result = addr::add(2, 2);
    assert_eq!(result, 4);
    //can also test this specifically with: cargo test --test integrated_tests
}