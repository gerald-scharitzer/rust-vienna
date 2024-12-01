pub fn assign(left: i32, right: i32) -> i32 {
    let source = right;
    let mut target = left;
    target = source;
    target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = assign(2, 4);
        assert_eq!(result, 4);
    }
}
