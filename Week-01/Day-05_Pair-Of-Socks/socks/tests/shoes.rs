use socks::dep::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_socks() {
        assert_eq!(0, pairing(""));
    }

    #[test]
    fn some_socks() {
        assert_eq!(2, pairing("ABABC"));
    }

    #[test]
    fn socks_bananza() {
        assert_eq!(4, pairing("CABBACCC"));
    }
}
