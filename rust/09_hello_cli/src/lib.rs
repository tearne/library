pub fn say_hello() {
    let mut numbers = vec![0; 99999];

    for j in 1..99{
        for i in 1..99 {
            numbers[i] = i * j;
        }
    }

    println!("Hello");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
