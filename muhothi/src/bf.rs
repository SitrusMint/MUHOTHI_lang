pub fn execute_bf(s :&String) -> Result<(), &str> {
    let mut buf = [0usize; 30000];

    let mut count :usize = 0;
    let mut i :usize = 0;
    let mut note: usize;
    while count != s.len() {
        match s.chars().nth(count).unwrap() {
            '+' => buf[i] += 1,
            '-' => buf[i] -= 1,
            '>' => i += 1,
            '<' => i -= 1,
            '.' => print!("{}", buf[i] as u8 as char),
            ',' => return Err("未実装"),
            '[' => if buf[i] == 0 {
                while s.chars().nth(count).unwrap() != ']' {
                    count += 1;
                }
            },
            ']' => note = i,
            _  => continue, /* ignore */
        };
        count += 1;
    }
    Ok(())
}