// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        let mut x: Vec<i32> = Vec::new();
        x.push(1);
        x.push(2);
        x.push(3);
        assert_eq!(x, vec![1, 2, 3]);
    }
}
