#[cfg(test)]
mod tests {
    use rust_the_forth::{
        stack::Stack,
        tokens::{read_tokens, tokenize},
        words::dictionary::WordsDictionary,
    };
    const TEST_STACK_SIZE: usize = 1024 * 128;

    #[test]
    fn test_add_sub() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(TEST_STACK_SIZE);

        read_tokens(&tokenize("1 2 + 4 -"), &mut test_stack, &mut dict);
        assert_eq!(test_stack.data, &[-1]);
    }

    #[test]
    fn test_mul_div() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(TEST_STACK_SIZE);

        read_tokens(&tokenize("2 4 * 3 /"), &mut test_stack, &mut dict);
        assert_eq!(test_stack.data, &[2]);
    }

    #[test]
    fn test_mul_add() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(TEST_STACK_SIZE);

        read_tokens(&tokenize("1 3 4 * +"), &mut test_stack, &mut dict);
        assert_eq!(test_stack.data, &[13]);
    }

    #[test]
    fn test_add_mul() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(TEST_STACK_SIZE);

        read_tokens(&tokenize("1 3 4 + *"), &mut test_stack, &mut dict);
        assert_eq!(test_stack.data, &[7]);
    }

    #[test]
    fn test_unit_computation_1() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(TEST_STACK_SIZE);

        read_tokens(&tokenize(": meter 100 * ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": decimeter 10 * ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": centimeter 1 * ;"), &mut test_stack, &mut dict);
        read_tokens(
            &tokenize("1 meter 5 decimeter 2 centimeter + +"),
            &mut test_stack,
            &mut dict,
        );
        assert_eq!(test_stack.data, &[152]);
    }

    #[test]
    fn test_unit_computation_2() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(TEST_STACK_SIZE);

        read_tokens(&tokenize(": seconds 1 * ;"), &mut test_stack, &mut dict);
        read_tokens(
            &tokenize(": minutes 60 * seconds ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": hours 60 * minutes ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize("2 hours 13 minutes 5 seconds + +"),
            &mut test_stack,
            &mut dict,
        );
        assert_eq!(test_stack.data, &[7985]);
    }

    #[test]
    fn test_constant_summation() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(TEST_STACK_SIZE);

        read_tokens(&tokenize(": one1 1 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": one2 one1 one1 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": one4 one2 one2 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": one8 one4 one4 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": one16 one8 one8 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": add1 + ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": add2 add1 add1 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": add4 add2 add2 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": add8 add4 add4 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": add16 add8 add8 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize("0 one16 add16"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[16]);
    }

    #[test]
    fn test_linear_summation() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(TEST_STACK_SIZE);

        read_tokens(&tokenize(": next1 dup 1 + ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": one1 1 ;"), &mut test_stack, &mut dict);
        read_tokens(
            &tokenize(": next2 next1 next1 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": next4 next2 next2 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": next8 next4 next4 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": next16 next8 next8 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(&tokenize(": add1 + ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": add2 add1 add1 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": add4 add2 add2 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": add8 add4 add4 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": add16 add8 add8 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize("0 next16 add16"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[136]);
    }

    #[test]
    fn test_geometric_summation() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(TEST_STACK_SIZE);

        read_tokens(&tokenize(": next1 dup 2 * ;"), &mut test_stack, &mut dict);
        read_tokens(
            &tokenize(": next2 next1 next1 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": next4 next2 next2 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": next8 next4 next4 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(&tokenize(": add1 + ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": add2 add1 add1 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": add4 add2 add2 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": add8 add4 add4 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize("1 next8 add8"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[511]);
    }

    #[test]
    fn test_power_of_2() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(TEST_STACK_SIZE);

        read_tokens(&tokenize(": next1 dup 2 * ;"), &mut test_stack, &mut dict);
        read_tokens(
            &tokenize(": next2 next1 next1 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": next4 next2 next2 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(&tokenize(": mul1 * ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": mul2 mul1 mul1 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize(": mul4 mul2 mul2 ;"), &mut test_stack, &mut dict);
        read_tokens(&tokenize("1 next4 mul4"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[1024]);
    }

    #[test]
    fn test_digit_to_string() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(TEST_STACK_SIZE);

        read_tokens(
            &tokenize(
                ": f dup 0 = if drop .\" zero\" else dup 1 = if drop .\" one\" else dup 2 = if drop .\" two\" then then then ;",
            ),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(&tokenize("0 f cr"), &mut test_stack, &mut dict);
        read_tokens(&tokenize("1 f cr"), &mut test_stack, &mut dict);
        read_tokens(&tokenize("2 f cr"), &mut test_stack, &mut dict);
        assert!(test_stack.data.is_empty());
    }
    #[test]
    fn test_heavy_word_definition() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(TEST_STACK_SIZE);
        read_tokens(&tokenize(": word1 1 ;"), &mut test_stack, &mut dict);
        read_tokens(
            &tokenize(": word2 word1 word1 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word4 word2 word2 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word8 word4 word4 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word16 word8 word8 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word32 word16 word16 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word64 word32 word32 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word128 word64 word64 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word256 word128 word128 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word512 word256 word256 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word1024 word512 word512 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word2048 word1024 word1024 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word4096 word2048 word2048 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word8192 word4096 word4096 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word16384 word8192 word8192 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word32768 word16384 word16384 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word65536 word32768 word32768 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word131072 word65536 word65536 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word262144 word131072 word131072 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word524288 word262144 word262144 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word1048576 word524288 word524288 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word2097152 word1048576 word1048576 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word4194304 word2097152 word2097152 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word8388608 word4194304 word4194304 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word16777216 word8388608 word8388608 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word33554432 word16777216 word16777216 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word67108864 word33554432 word33554432 ;"),
            &mut test_stack,
            &mut dict,
        );
        read_tokens(
            &tokenize(": word134217728 word67108864 word67108864 ;"),
            &mut test_stack,
            &mut dict,
        );

        assert!(test_stack.data.is_empty());
    }
}
