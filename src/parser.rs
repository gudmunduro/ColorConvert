use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while_m_n},
    character::complete::char,
    combinator::{map, map_res, opt},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

#[derive(PartialEq, Debug)]
pub struct StandardColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl StandardColor {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        StandardColor {
            red,
            green,
            blue,
            alpha,
        }
    }
}

fn hex_to_u8(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn int_to_u8(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 10)
}

fn float_to_u8_color(value: f64) -> u8 {
    (value * 255.0).round() as u8
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn is_int_digit(c: char) -> bool {
    c.is_digit(10)
}

fn is_float_digit(c: char) -> bool {
    is_int_digit(c) || c == '.'
}

fn sp(i: &str) -> IResult<&str, &str> {
    let chars = " \t\r\n";

    take_while(move |c| chars.contains(c))(i)
}

fn hex_u8_value(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), hex_to_u8)(input)
}

fn int_u8_value(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(1, 3, is_int_digit), int_to_u8)(input)
}

fn float_value(input: &str) -> IResult<&str, f64> {
    map_res(take_while_m_n(1, 50, is_float_digit), str::parse)(input)
}

fn single_tuple_int_value(input: &str) -> IResult<&str, u8> {
    preceded(
        opt(sp),
        terminated(int_u8_value, pair(opt(sp), opt(char(',')))),
    )(input)
}

fn single_tuple_float_value(input: &str) -> IResult<&str, f64> {
    preceded(
        opt(sp),
        terminated(float_value, pair(opt(sp), opt(char(',')))),
    )(input)
}

fn parse_hex_rgb_value(input: &str) -> IResult<&str, StandardColor> {
    let (input, (red, green, blue, alpha)) = preceded(
        tag("#"),
        tuple((
            hex_u8_value,
            hex_u8_value,
            hex_u8_value,
            map(opt(hex_u8_value), |i| i.unwrap_or(255)),
        )),
    )(input)?;

    Ok((input, StandardColor::new(red, green, blue, alpha)))
}

fn parse_int_rgb(input: &str) -> IResult<&str, StandardColor> {
    let (input, (red, green, blue, alpha)) = preceded(
        pair(opt(alt((tag("rgba"), tag("rgb")))), char('(')),
        terminated(
            tuple((
                single_tuple_int_value,
                single_tuple_int_value,
                single_tuple_int_value,
                map(opt(single_tuple_int_value), |i| i.unwrap_or(255)),
            )),
            tag(")"),
        ),
    )(input)?;

    Ok((input, StandardColor::new(red, green, blue, alpha)))
}

fn parse_float_rgb(input: &str) -> IResult<&str, StandardColor> {
    let (input, (red, green, blue, alpha)) = preceded(
        pair(opt(alt((tag("rgba"), tag("rgb")))), char('(')),
        terminated(
            tuple((
                single_tuple_float_value,
                single_tuple_float_value,
                single_tuple_float_value,
                map(opt(single_tuple_float_value), |i| i.unwrap_or(1.0)),
            )),
            tag(")"),
        ),
    )(input)?;

    Ok((
        input,
        StandardColor::new(
            float_to_u8_color(red),
            float_to_u8_color(green),
            float_to_u8_color(blue),
            float_to_u8_color(alpha),
        ),
    ))
}

pub fn parse_color(input: &str) -> Result<StandardColor, Box<dyn std::error::Error + '_>> {
    let (_, color) = alt((parse_hex_rgb_value, parse_int_rgb, parse_float_rgb))(input)?;

    Ok(color)
}
