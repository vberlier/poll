use std::fmt;

pub trait Dimensions {
    fn width(&self) -> i32;
    fn height(&self) -> i32;
}

#[derive(Debug)]
pub struct Widget<T> {
    width: i32,
    height: i32,
    padding: i32,
    content: T,
}

impl<T: fmt::Display> Widget<T> {
    pub fn new(width: i32, height: i32, padding: i32, content: T) -> Self {
        Self {
            width,
            height,
            padding,
            content,
        }
    }
}

impl<T> Widget<T> {
    pub fn with_padding(mut self, padding: i32) -> Self {
        self.padding = padding;
        self
    }
}

pub fn wrap<T: fmt::Display + Dimensions>(content: T) -> Widget<T> {
    Widget::new(content.width(), content.height(), 0, content)
}

impl<T: fmt::Display> fmt::Display for Widget<T> {
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

        self.content.fmt(f)?;

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

#[derive(Debug)]
pub struct NumberBadge {
    value: i32,
}

impl NumberBadge {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

impl Dimensions for NumberBadge {
    fn width(&self) -> i32 {
        ((self.value as f64 + 0.5).log10().abs() as i32 + 1 + 2) * 8
    }

    fn height(&self) -> i32 {
        18
    }
}

impl fmt::Display for NumberBadge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self.width();
        let height = self.height();

        write!(
            f,
            r###"
                <style>
                .text {{
                    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Helvetica,Arial,sans-serif,"Apple Color Emoji","Segoe UI Emoji";
                    fill: #32383f;
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
                    fill="#afb8c1"
                    stroke="#afb8c1"
                    stroke-width="2"
                    rx="5"
                />
                <text class="text" x="8" y="14">{count}</text>
            "###,
            width = width,
            height = height,
            count = self.value
        )
    }
}

#[derive(Debug)]
pub struct HorizontalBar {
    width: i32,
    height: i32,
    count: i32,
    total: i32,
}

impl HorizontalBar {
    pub fn new(width: i32, height: i32, count: i32, total: i32) -> Self {
        Self {
            width,
            height,
            count,
            total,
        }
    }
}

impl Dimensions for HorizontalBar {
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }
}

impl fmt::Display for HorizontalBar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self.width();
        let height = self.height();

        let fill_width = match self.total {
            0 => 0.0,
            _ => (self.count as f64) / (self.total as f64) * (width as f64),
        };

        let empty_width = (width as f64) - fill_width;

        write!(
            f,
            r###"
                <mask id="bar-mask">
                    <rect x="0" y="0" width="{width}" height="{height}" fill="white" rx="5" />
                </mask>
                <rect
                    mask="url(#bar-mask)"
                    x="0"
                    y="0"
                    width="{fill_width}"
                    height="{height}"
                    fill="#0969da"
                />
                <rect
                    mask="url(#bar-mask)"
                    x="{fill_width}"
                    y="0"
                    width="{empty_width}"
                    height="{height}"
                    fill="#80ccff"
                />
            "###,
            width = width,
            height = height,
            fill_width = fill_width,
            empty_width = empty_width,
        )
    }
}
