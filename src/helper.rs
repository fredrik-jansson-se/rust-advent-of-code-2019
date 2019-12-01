use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, opt};
use nom::sequence::pair;
use nom::IResult;

fn signed_to_val(s: (Option<&str>, &str)) -> Result<i32, std::num::ParseIntError> {
    match s.0 {
        Some("-") => s.1.parse::<i32>().map(|v| -v),
        _ => s.1.parse::<i32>(),
    }
}

pub fn i32_val(i: &str) -> IResult<&str, i32> {
    let a = alt((tag("-"), tag("+")));
    map_res(pair(opt(a), digit1), signed_to_val)(i)
}

pub fn u32_val(i: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(i)
}

pub fn usize_val(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn helper_parse_int32() {
        assert_eq!(i32_val("123"), Ok(("", 123)));
        assert_eq!(i32_val("+123"), Ok(("", 123)));
        assert_eq!(i32_val("-123"), Ok(("", -123)));
    }

    #[test]
    fn helper_parse_uint32() {
        assert_eq!(u32_val("123"), Ok(("", 123)));
    }
}
