use std::fmt;

use worker::{Response, Result};

pub trait Widget {
    fn format_response(self) -> Result<Response>;
}

impl<T> Widget for T
where
    T: ToString,
{
    fn format_response(self) -> Result<Response> {
        let mut response = Response::ok(self.to_string())?;
        response
            .headers_mut()
            .append("content-type", "image/svg+xml; charset=utf-8")?;
        Ok(response)
    }
}

#[derive(Debug)]
pub struct CountWidget {
    value: i32,
}

impl CountWidget {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

impl fmt::Display for CountWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let digits = (self.value as f64 + 0.5).log10().abs() as i32 + 1;

        let image_width = digits * 9;
        let image_height = 24;

        write!(
            f,
            r###"
                <svg
                    width="{image_width}"
                    height="{image_height}"
                    viewBox="0 0 {image_width} {image_height}"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <style>
                    .text {{
                        font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Helvetica,Arial,sans-serif,"Apple Color Emoji","Segoe UI Emoji";
                        fill: currentColor;
                        line-height: 1.5;
                        font-variant-numeric: tabular-nums;
                    }}
                    </style>
                    <text class="text" x="0" y="18">{value}</text>
                </svg>
            "###,
            image_width = image_width,
            image_height = image_height,
            value = self.value
        )
    }
}
