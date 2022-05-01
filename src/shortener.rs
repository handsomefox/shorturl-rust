pub fn make(link: &str) -> String {
    hash(link)
}

#[allow(arithmetic_overflow)]
fn hash(s: &str) -> String {
    let mut h: u64 = 5381;
    let bytes: Vec<u8> = Vec::from(s);

    for c in bytes.iter() {
        h = ((h << 5) + h) + *c as u64;
    }
    let hex = format!("{:x}", h);
    hex
}
mod tests {
    use crate::shortener::hash;

    #[test]
    fn hash_test() {
        let hex = hash("string");
        assert_eq!("6531c93affc", hex);
    }
}
