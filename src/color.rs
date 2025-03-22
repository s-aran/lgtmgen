use regex::bytes::Regex;

pub(crate) type RgbColor = (u8, u8, u8);

pub(crate) fn convert_color_string_to_rgb(color: &String) -> Result<RgbColor, String> {
    let lowered = color.to_lowercase();

    // #FFFFFF
    {
        static RGB_PATTERN: &str = "^#?([0-9a-f]{6})$";
        let re = Regex::new(RGB_PATTERN).unwrap();
        let captures = re.captures(&lowered.as_bytes());
        if captures.is_some() {
            let c = captures.unwrap();

            if c.get(1).unwrap().len() == 6 {
                let raw_bytes = c.get(1).unwrap().to_owned().as_bytes();

                let maybe_r = std::str::from_utf8(&raw_bytes[0..2]);
                let maybe_g = std::str::from_utf8(&raw_bytes[2..4]);
                let maybe_b = std::str::from_utf8(&raw_bytes[4..]);

                let r = match maybe_r {
                    Ok(d) => u8::from_str_radix(d, 16).unwrap_or(0),
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };

                let g = match maybe_g {
                    Ok(d) => u8::from_str_radix(d, 16).unwrap_or(0),
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };

                let b = match maybe_b {
                    Ok(d) => u8::from_str_radix(d, 16).unwrap_or(0),
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };

                return Ok((r, g, b));
            }
        }
    }

    // #FFF
    {
        static RGB_PATTERN: &str = "^#?([0-9a-f]{3})$";
        let re = Regex::new(RGB_PATTERN).unwrap();
        let captures = re.captures(&lowered.as_bytes());
        if captures.is_some() {
            let c = captures.unwrap();

            if c.get(1).unwrap().len() == 3 {
                let raw_bytes = c.get(1).unwrap().to_owned().as_bytes();

                let maybe_r = std::str::from_utf8(&raw_bytes[0..1]);
                let maybe_g = std::str::from_utf8(&raw_bytes[1..2]);
                let maybe_b = std::str::from_utf8(&raw_bytes[2..]);

                let r = match maybe_r {
                    Ok(d) => u8::from_str_radix(format!("{}{}", d, d).as_str(), 16).unwrap_or(0),
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };

                let g = match maybe_g {
                    Ok(d) => u8::from_str_radix(format!("{}{}", d, d).as_str(), 16).unwrap_or(0),
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };

                let b = match maybe_b {
                    Ok(d) => u8::from_str_radix(format!("{}{}", d, d).as_str(), 16).unwrap_or(0),
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };

                return Ok((r, g, b));
            }
        }
    }

    let rgb = match color.as_str() {
        "black" => (0, 0, 0),
        "white" => (255, 255, 255),
        "red" => (255, 0, 0),
        "green" => (0, 255, 0),
        "blue" => (0, 0, 255),
        "yellow" => (255, 255, 0),
        "cyan" => (0, 255, 255),
        "magenta" => (255, 0, 255),
        _ => return Err(format!("invalid color {}", color)),
    };

    Ok(rgb)
}

#[cfg(test)]
mod tests {
    use crate::color::convert_color_string_to_rgb;

    #[test]
    fn test_color_6() {
        {
            let color = "#FF0F00".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((255, 15, 0)));
        }

        {
            let color = "00FF0F".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((0, 255, 15)));
        }
    }

    #[test]
    fn test_color_3() {
        {
            let color = "#F10".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((255, 17, 0)));
        }

        {
            let color = "0F1".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((0, 255, 17)));
        }
    }

    #[test]
    fn test_color_name() {
        {
            let color = "black".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((0, 0, 0)));
        }
        {
            let color = "white".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((255, 255, 255)));
        }
        {
            let color = "red".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((255, 0, 0)));
        }
        {
            let color = "green".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((0, 255, 0)));
        }
        {
            let color = "blue".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((0, 0, 255)));
        }
        {
            let color = "yellow".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((255, 255, 0)));
        }
        {
            let color = "cyan".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((0, 255, 255)));
        }
        {
            let color = "magenta".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((255, 0, 255)));
        }
    }
}
