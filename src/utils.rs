use std::io::stdin;



pub fn wait_for_input() {
    let mut s = String::new();
    let _  = stdin().read_line(&mut s);
}
