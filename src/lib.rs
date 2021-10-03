use std::convert::TryFrom;

use widgets::render_count;
use worker::*;

mod params;
mod utils;
mod widgets;

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

    let fill_width = match total {
        0 => 0.0,
        _ => (count as f64) / (total as f64) * (width as f64),
    };

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
                    fill="#0969da"
                />
                <rect
                    mask="url(#bar-mask)"
                    x="{position}"
                    y="{padding}"
                    width="{empty_width}"
                    height="{height}"
                    fill="#80ccff"
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
        .get("/", |_, _| {
            let mut response = Response::empty()?.with_status(302);
            response
                .headers_mut()
                .append("Location", "https://github.com/vberlier/poll")?;
            Ok(response)
        })
        .get_async("/vote", |req, ctx| async move {
            // Return to the previous page with `history.back()` if `redirect` is not specified.
            let mut response = Response::ok("<script>history.back()</script>")?;
            response
                .headers_mut()
                .append("content-type", "text/html; charset=utf-8")?;

            if let Ok(url) = req.url() {
                if let Some((_, redirect)) = url.query_pairs().find(|(key, _)| key == "redirect") {
                    response = Response::empty()?.with_status(302);
                    response
                        .headers_mut()
                        .append("location", redirect.as_ref())?;
                }
            }

            response
                .headers_mut()
                .append("cache-control", "private, max-age=0, no-cache")?;

            let store = ctx.kv("POLL")?;

            let ip = match req.headers().get("x-real-ip")? {
                Some(ip) => ip,
                None => return Ok(response),
            };

            if let Some((poll, Some(option))) = get_poll_parameter(&req) {
                // Create anonymized voter identifier based on ip and poll id.
                let voter_id = base64::encode(
                    blake3::Hasher::new()
                        .update(ip.as_bytes())
                        .update(b"$")
                        .update(poll.as_bytes())
                        .update(b"$")
                        .update(ctx.secret("secret_key")?.to_string().as_bytes())
                        .finalize()
                        .as_bytes(),
                );

                console_log!("vote {}={} ({})", poll, option, voter_id);

                let key_voted = format!("voted:{}:{}", poll, voter_id);
                let voted_flag = store
                    .get(&key_voted)
                    .await?
                    .and_then(|value| value.as_json::<i32>().ok())
                    .unwrap_or(0);

                // Return early if already voted.
                if voted_flag != 0 {
                    return Ok(response);
                }

                // Increment both the total amount of votes and the count for the selected option.
                for key in [
                    format!("total:{}", poll),
                    format!("count:{}:{}", poll, option),
                ] {
                    let value = store
                        .get(&key)
                        .await?
                        .and_then(|value| value.as_json::<i32>().ok())
                        .unwrap_or(0);

                    console_log!("{} = {}", key, value);

                    store.put(&key, value + 1)?.execute().await?;

                    console_log!("{} = {}", key, value + 1);
                }

                // Set the voted flag for this user.
                store.put(&key_voted, 1)?.execute().await?;
            }

            Ok(response)
        })
        .get_async("/show", |req, ctx| async move {
            let width = 300;
            let height = 12;
            let padding = 2;

            let mut svg = render_svg(width, height, padding, 0, 0);

            if let Some((poll, Some(option))) = get_poll_parameter(&req) {
                console_log!("{}={}", poll, option);

                let store = ctx.kv("POLL")?;

                let key_total = format!("total:{}", poll);
                let total = store
                    .get(&key_total)
                    .await?
                    .and_then(|value| value.as_json::<i32>().ok())
                    .unwrap_or(0);

                console_log!("{} = {}", key_total, total);

                let key_count = format!("count:{}:{}", poll, option);
                let count = store
                    .get(&key_count)
                    .await?
                    .and_then(|value| value.as_json::<i32>().ok())
                    .unwrap_or(0);

                console_log!("{} = {}", key_count, count);

                svg = render_svg(width, height, padding, count, total);
            }

            let mut headers = Headers::new();
            headers.append("content-type", "image/svg+xml; charset=utf-8")?;
            headers.append("cache-control", "private, max-age=0, no-cache")?;

            Ok(Response::ok(svg)?.with_headers(headers))
        })
        .get_async("/count", |req, ctx| async move {
            let mut headers = Headers::new();
            headers.append("content-type", "image/svg+xml; charset=utf-8")?;
            headers.append("cache-control", "private, max-age=0, no-cache")?;

            if let Ok(common) = params::CommonParams::try_from(&req) {
                let store = ctx.kv("POLL")?;

                let key = if let Some(option) = common.option {
                    format!("count:{}:{}", common.poll, option)
                } else {
                    format!("total:{}", common.poll)
                };

                let count = store
                    .get(&key)
                    .await?
                    .and_then(|value| value.as_json::<i32>().ok())
                    .unwrap_or(0);

                Ok(Response::ok(render_count(count))?.with_headers(headers))
            } else {
                Ok(Response::ok(render_count(0))?.with_headers(headers))
            }
        })
        .run(req, env)
        .await
}
