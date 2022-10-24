fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        assert_eq!(2 + 2, 4);
    }
}
