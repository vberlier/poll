use std::convert::TryFrom;

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
            let mut headers = Headers::new();
            headers.append("content-type", "text/html; charset=utf-8")?;
            headers.append("cache-control", "public, max-age=15")?;

            // Return to the previous page with `history.back()` by default.
            let mut response =
                Response::ok("<script>history.back()</script>")?.with_headers(headers.clone());

            if let Ok(submission) = params::VoteParams::try_from(&req) {
                // If the user specified a redirect query parameter return a 302 instead.
                if let Some(redirect) = submission.common.query.get("redirect") {
                    headers.append("location", redirect)?;
                    response = Response::empty()?.with_status(302).with_headers(headers);
                }

                let store = ctx.kv("POLL")?;

                let voter_id =
                    submission.create_anonymized_voter_id(&ctx.secret("secret_key")?.to_string());

                let voted_flag_key = format!("voted:{}:{}", submission.common.poll, voter_id);
                let voted_flag = store
                    .get(&voted_flag_key)
                    .await?
                    .and_then(|value| value.as_json::<i32>().ok())
                    .unwrap_or(0);

                // Return early if already voted.
                if voted_flag != 0 {
                    return Ok(response);
                }

                // Increment both the total amount of votes and the count for the selected option.
                for key in [
                    format!("total:{}", submission.common.poll),
                    format!("count:{}:{}", submission.common.poll, submission.vote),
                ] {
                    let value = store
                        .get(&key)
                        .await?
                        .and_then(|value| value.as_json::<i32>().ok())
                        .unwrap_or(0);

                    store.put(&key, value + 1)?.execute().await?;
                }

                // Set the voted flag to prevent another submission.
                store.put(&voted_flag_key, 1)?.execute().await?;
            }

            Ok(response)
        })
        .get_async("/show", |req, ctx| async move {
            let mut headers = Headers::new();
            headers.append("content-type", "image/svg+xml; charset=utf-8")?;
            headers.append("cache-control", "public, max-age=15")?;

            if let Ok(common) = params::CommonParams::try_from(&req) {
                if let Some(option) = common.option {
                    let store = ctx.kv("POLL")?;

                    let total = store
                        .get(&format!("total:{}", common.poll))
                        .await?
                        .and_then(|value| value.as_json::<i32>().ok())
                        .unwrap_or(0);

                    let count = store
                        .get(&format!("count:{}:{}", common.poll, option))
                        .await?
                        .and_then(|value| value.as_json::<i32>().ok())
                        .unwrap_or(0);

                    return Ok(
                        Response::ok(widgets::render_bar(count, total))?.with_headers(headers)
                    );
                }
            }

            Ok(Response::ok(widgets::render_bar(0, 0))?.with_headers(headers))
        })
        .get_async("/count", |req, ctx| async move {
            let mut headers = Headers::new();
            headers.append("content-type", "image/svg+xml; charset=utf-8")?;
            headers.append("cache-control", "public, max-age=15")?;

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

                Ok(Response::ok(widgets::render_count(count))?.with_headers(headers))
            } else {
                Ok(Response::ok(widgets::render_count(0))?.with_headers(headers))
            }
        })
        .run(req, env)
        .await
}
