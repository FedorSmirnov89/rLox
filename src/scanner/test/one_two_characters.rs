use crate::domain::scanning::TokenType;
use crate::{operator, splitting_test};

use super::{
    test_one_char_not_spaced, test_one_char_spaced, test_two_chars_not_spaced,
    test_two_chars_spaced,
};

// one char
splitting_test!(bang, spaced);
splitting_test!(bang, not spaced);
splitting_test!(equal, spaced);
splitting_test!(equal, not spaced);
splitting_test!(less, spaced);
splitting_test!(less, not spaced);
splitting_test!(greater, spaced);
splitting_test!(greater, not spaced);

// two chars
splitting_test!(less_equal, spaced);
splitting_test!(less_equal, not spaced);
splitting_test!(greater_equal, spaced);
splitting_test!(greater_equal, not spaced);
splitting_test!(bang_equal, spaced);
splitting_test!(bang_equal, not spaced);
splitting_test!(equal_equal, spaced);
splitting_test!(equal_equal, not spaced);
