#[allow(unused)]

fn main() {
    let a = "Hello, World!";
    let b = a.into();

    print_string(b);
}

fn print_string(s: String){
    println!("{}", s)
}
