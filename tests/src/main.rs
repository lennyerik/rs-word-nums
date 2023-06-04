fn main() {}

#[cfg(test)]
mod tests {
    use word_nums::num;

    #[test]
    fn test_single_digits() {
        assert_eq!(num!(one), 1i8);
        assert_eq!(num!(two), 2i8);
        assert_eq!(num!(three), 3i8);
        assert_eq!(num!(four), 4i8);
        assert_eq!(num!(five), 5i8);
        assert_eq!(num!(six), 6i8);
        assert_eq!(num!(seven), 7i8);
        assert_eq!(num!(eight), 8i8);
        assert_eq!(num!(nine), 9i8);
    }

    #[test]
    fn test_double_digit() {
        assert_eq!(num!(ten), 10i8);
        assert_eq!(num!(twenty one), 21i8);
        assert_eq!(num!(thirty two), 32i8);
        assert_eq!(num!(fourty), 40i8);
        assert_eq!(num!(fifty six), 56i8);
        assert_eq!(num!(sixty seven), 67i8);
        assert_eq!(num!(seventy seven), 77i8);
        assert_eq!(num!(eighty one), 81i8);
        assert_eq!(num!(ninety three), 93i8);
    }

    #[test]
    fn test_negative() {
        assert_eq!(num!(minus ten), -10i8);
        assert_eq!(
            num!(negative one thousand three hundred thirty seven),
            -1337i16
        );
    }

    #[test]
    fn test_explicit_unsigned() {
        assert_eq!(num!(plus one thousand three hundred thirty seven), 1337u16);
        assert_eq!(
            num!(positive seven thousand three hundred thirty one),
            7331u16
        );
    }

    #[test]
    fn test_implicit_one() {
        assert_eq!(num!(hundred twenty three), 123i8);
        assert_eq!(num!(thousand three hundred thirty seven), 1337i16);
    }

    #[test]
    fn test_a() {
        assert_eq!(num!(a hundred twenty three), 123i8);
    }

    #[test]
    fn test_and() {
        assert_eq!(num!(one thousand three hundred and thirty seven), 1337i16);
    }

    #[test]
    fn test_thousands_as_hundreds() {
        assert_eq!(num!(thirteen hundred thirty seven), 1337i16);
    }

    #[test]
    fn test_number_types() {
        assert_eq!(num!(one hundred twenty seven), 127i8);
        assert_eq!(num!(plus two hundred fifty five), 255u8);
        assert_eq!(
            num!(thirty two thousand seven hundred sixty seven),
            32767i16
        );
        assert_eq!(
            num!(plus sixty five thousand five hundred thirty five),
            65535u16
        );
        assert_eq!(
            num!(two billion one hundred forty seven million four hundred eighty-three thousand six hundred forty seven),
            2147483647i32
        );
        assert_eq!(
            num!(plus four billion two hundred ninety four million nine hundred sixty seven thousand two hundred ninety five),
            4294967295u32
        );
        assert_eq!(
            num!(nine quintillion two hundred twenty three quadrillion three hundred seventy two trillion thirty six billion eight hundred fifty four million seven hundred seventy five thousand eight hundred seven),
            9223372036854775807i64
        );
        assert_eq!(
            num!(plus eighteen quintillion four hundred forty-six quadrillion seven hundred forty four trillion seventy three billion seven hundred nine million five hundred fifty one thousand six hundred fifteen
            ),
            18446744073709551615u64
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(num!(zero), 0i8);
        assert_eq!(num!(plus zero), 0u8);
        assert_eq!(num!(minus zero), 0i8);
    }
}
