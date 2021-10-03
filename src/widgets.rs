use std::fmt;

#[derive(Debug)]
struct WidgetWrapper<F> {
    width: i32,
    height: i32,
    padding: i32,
    content: F,
}

impl<F> WidgetWrapper<F>
where
    F: Fn(&mut fmt::Formatter) -> fmt::Result,
{
    pub fn new(width: i32, height: i32, padding: i32, content: F) -> Self {
        Self {
            width,
            height,
            padding,
            content,
        }
    }
}

impl<F> fmt::Display for WidgetWrapper<F>
where
    F: Fn(&mut fmt::Formatter) -> fmt::Result,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = self.width + self.padding * 2;
        let height = self.height + self.padding * 2;

        write!(
            f,
            r###"
                <svg
                    width="{width}"
                    height="{height}"
                    viewBox="0 0 {width} {height}"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <g transform="translate({padding}, {padding})">
            "###,
            width = width,
            height = height,
            padding = self.padding
        )?;

        (self.content)(f)?;

        write!(
            f,
            r###"
                    </g>
                </svg>
            "###
        )?;

        Ok(())
    }
}

pub fn render_count(count: i32) -> String {
    let digits = (count as f64 + 0.5).log10().abs() as i32 + 1;

    let width = (digits + 2) * 8;
    let height = 18;

    WidgetWrapper::new(width, height, 2, |f| {
        write!(
            f,
            r###"
                <style>
                .text {{
                    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Helvetica,Arial,sans-serif,"Apple Color Emoji","Segoe UI Emoji";
                    fill: #762c00;
                    font-size: 14px;
                    font-weight: bold;
                    line-height: 1.5;
                    font-variant-numeric: tabular-nums;
                }}
                </style>
                <rect
                    x="0"
                    y="0"
                    width="{width}"
                    height="{height}"
                    fill="#fb8f44"
                    stroke="#fb8f44"
                    stroke-width="2"
                    rx="5"
                />
                <text class="text" x="8" y="14">{count}</text>
            "###,
            width = width,
            height = height,
            count = count
        )
    }).to_string()
}
