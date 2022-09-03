use crate::{parser::StandardColor, TargetFormats};

fn foramt_int_rgb(color: &StandardColor) -> String {
    let StandardColor {
        red, green, blue, ..
    } = color;
    format!("rgb({red}, {green}, {blue})")
}

fn foramt_int_rgba(color: &StandardColor) -> String {
    let StandardColor {
        red,
        green,
        blue,
        alpha,
    } = color;
    format!("rgba({red}, {green}, {blue}, {alpha})")
}

fn int_color_to_float(color: u8) -> f64 {
    (color as f64) / 255.0
}

fn foramt_float_rgb(color: &StandardColor) -> String {
    let StandardColor {
        red, green, blue, ..
    } = color;
    let (red, green, blue) = (
        int_color_to_float(*red),
        int_color_to_float(*green),
        int_color_to_float(*blue),
    );

    format!("rgb({red:.2}, {green:.2}, {blue:.2})")
}

fn foramt_float_rgba(color: &StandardColor) -> String {
    let StandardColor {
        red, green, blue, alpha
    } = color;
    let (red, green, blue, alpha) = (
        int_color_to_float(*red),
        int_color_to_float(*green),
        int_color_to_float(*blue),
        int_color_to_float(*alpha),
    );
    
    format!("rgba({red:.2}, {green:.2}, {blue:.2}, {alpha:.2})")
}

fn format_hex_rgb(color: &StandardColor) -> String {
    let StandardColor {
        red, green, blue, ..
    } = color;
    format!("#{red:02X}{green:02X}{blue:02X}")
}

fn format_hex_rgba(color: &StandardColor) -> String {
    let StandardColor {
        red, green, blue, alpha
    } = color;
    format!("#{red:02X}{green:02X}{blue:02X}{alpha:02X}")
}

pub fn format_color(target_format: &TargetFormats, color: &StandardColor) -> String {
    use TargetFormats::*;
    match target_format {
        Intrgb => foramt_int_rgb(color),
        Intrgba => foramt_int_rgba(color),
        Floatrgb => foramt_float_rgb(color),
        Floatrgba => foramt_float_rgba(color),
        Hexrgb => format_hex_rgb(color),
        Hexrgba => format_hex_rgba(color)
    }
}
