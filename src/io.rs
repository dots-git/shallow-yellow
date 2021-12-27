pub fn read() -> String {
    use std::io::stdin;
    let mut input: String = "".to_owned();
    stdin().read_line(&mut input).expect("Error reading stdin");
    input
}