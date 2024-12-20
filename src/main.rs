fn main() {
    add(2, 2);
    println!("Hello, rust!");
}

fn add(first: i32, second: i32) -> i32 {
    first + second
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = super::add(2, 2);
        assert_eq!(result, 4);
    }
}
