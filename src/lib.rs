use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

fn get_poll_parameter(req: &Request) -> Option<(String, Option<String>)> {
    req.url()
        .ok()?
        .query_pairs()
        .find(|(key, _)| {
            key.split_once('.')
                .map(|(scope, name)| !scope.is_empty() && !name.is_empty())
                .unwrap_or(false)
        })
        .map(|(key, value)| {
            (
                String::from(key),
                if value.is_empty() {
                    None
                } else {
                    Some(String::from(value))
                },
            )
        })
}

fn render_svg(width: i32, height: i32, padding: i32, count: i32, total: i32) -> String {
    let image_width = width + 2 * padding;
    let image_height = height + 2 * padding;

    let fill_width = (count as f64) / (total as f64) * (width as f64);
    let empty_width = (width as f64) - fill_width;
    let position = (padding as f64) + fill_width;

    format!(
        r###"
            <svg
                width="{image_width}"
                height="{image_height}"
                viewBox="0 0 {image_width} {image_height}"
                xmlns="http://www.w3.org/2000/svg"
            >
                <mask id="bar-mask">
                    <rect x="{padding}" y="{padding}" width="{width}" height="{height}" fill="white" rx="5" />
                </mask>
                <rect
                    mask="url(#bar-mask)"
                    x="{padding}"
                    y="{padding}"
                    width="{fill_width}"
                    height="{height}"
                    fill="#2563EB"
                />
                <rect
                    mask="url(#bar-mask)"
                    x="{position}"
                    y="{padding}"
                    width="{empty_width}"
                    height="{height}"
                    fill="#93C5FD"
                />
            </svg>
        "###,
        width = width,
        height = height,
        image_width = image_width,
        image_height = image_height,
        padding = padding,
        fill_width = fill_width,
        empty_width = empty_width,
        position = position
    )
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    log_request(&req);

    // Get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/vote", |req, _| {
            // Return to the previous page with `history.back()` if `redirect` is not specified.
            let mut response = Response::ok("<script>history.back()</script>")?;
            response
                .headers_mut()
                .append("Content-Type", "text/html; charset=utf-8")?;

            if let Ok(url) = req.url() {
                if let Some((_, redirect)) = url.query_pairs().find(|(key, _)| key == "redirect") {
                    response = Response::empty()?.with_status(302);
                    response
                        .headers_mut()
                        .append("Location", redirect.as_ref())?;
                }
            }

            response
                .headers_mut()
                .append("Cache-Control", "private, max-age=0, no-cache")?;

            if let Some((poll, option)) = get_poll_parameter(&req) {
                console_log!("{}={}", poll, option.unwrap_or("<no option>".into()));
            }

            Ok(response)
        })
        .get("/show", |req, _| {
            let mut headers = Headers::new();
            headers.append("Cache-Control", "private, max-age=0, no-cache")?;
            headers.append("Content-Type", "image/svg+xml; charset=utf-8")?;

            if let Some((poll, option)) = get_poll_parameter(&req) {
                console_log!("{}={}", poll, option.unwrap_or("<no option>".into()));
            }

            let svg = render_svg(300, 12, 2, 52, 70);

            Ok(Response::ok(svg)?.with_headers(headers))
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .run(req, env)
        .await
}
