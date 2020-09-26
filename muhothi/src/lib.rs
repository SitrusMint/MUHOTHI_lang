pub fn execute_bf(s :&String) -> Result<(), String> {
    let mut buf = [0usize; 30000];

    let mut count :usize = 0;
    let mut i :usize = 0;
    // let mut note: usize;
    while count != s.len() {
        match s.chars().nth(count).unwrap() {
            '+' => buf[i] += 1,
            '-' => buf[i] -= 1,
            '>' => i += 1,
            '<' => i -= 1,
            '.' => print!("{}", buf[i] as u8 as char),
            // TODO ここどうにかしないと
            ',' => continue, 
            // TODO どうすんねん
            '[' => continue,
            // TODO 難しすぎ
            ']' => continue,
            _  => continue, 
        };
        count += 1;
    }
    Ok(())
}


#[cfg(test)]
mod test {
    use super::{execute_bf};
    #[test]
    fn test_bf() {
        let s = "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.+.>++++++++++."
            .to_string();
        match execute_bf(&s) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}