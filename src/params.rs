use std::convert::TryFrom;
use std::{collections::HashMap, convert::TryInto};
use url::Url;
use worker::{Error, Request};

#[derive(Debug)]
pub struct CommonParams {
    pub url: Url,
    pub poll: String,
    pub option: Option<String>,
    pub query: HashMap<String, String>,
}

impl TryFrom<&Request> for CommonParams {
    type Error = Error;

    fn try_from(req: &Request) -> Result<Self, Self::Error> {
        let url = req.url()?;
        let query: HashMap<_, _> = url.query_pairs().into_owned().collect();

        let (poll, option) = query
            .iter()
            .find(|(key, _)| {
                key.split_once('.')
                    .map(|(scope, name)| !scope.is_empty() && !name.is_empty())
                    .unwrap_or(false)
            })
            .ok_or("Poll identifier required.")?;

        let option = if option.is_empty() {
            None
        } else {
            Some(option.into())
        };

        Ok(Self {
            url,
            poll: poll.into(),
            option,
            query,
        })
    }
}

#[derive(Debug)]
pub struct VoteParams {
    pub common: CommonParams,
    pub ip: String,
    pub country: String,
    pub vote: String,
}

impl VoteParams {
    pub fn create_anonymized_voter_id(&self, secret_key: &str) -> String {
        base64::encode(
            blake3::Hasher::new()
                .update(self.ip.as_bytes())
                .update(b"$")
                .update(self.common.poll.as_bytes())
                .update(b"$")
                .update(secret_key.as_bytes())
                .finalize()
                .as_bytes(),
        )
    }
}

impl TryFrom<&Request> for VoteParams {
    type Error = Error;

    fn try_from(req: &Request) -> Result<Self, Self::Error> {
        let common: CommonParams = req.try_into()?;

        let ip = req
            .headers()
            .get("x-real-ip")?
            .ok_or("Ip required to be able to vote.")?;

        let country = req
            .cf()
            .country()
            .ok_or("Country required to be able to vote.")?;

        let vote = common
            .option
            .clone()
            .ok_or("Poll option required to be able to vote.")?;

        Ok(Self {
            common,
            ip,
            country,
            vote,
        })
    }
}
