pub fn add(left: i32, right: i32) -> i32 {
    let source = right;
    let mut target = left;
    target += source; // assign needs mutable target
    target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 4);
        assert_eq!(result, 6);
    }
}
