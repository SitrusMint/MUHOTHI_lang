use super::bf::*;

#[test]
fn test_bf1() {
    let s = "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.+.>++++++++++."
        .to_string();
    match execute_bf(&s) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}

fn test_bf2() {
    // TODO
}