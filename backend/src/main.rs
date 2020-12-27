use std::env;

use env_logger;
use envy;
use log::info;
use slack::SlackConfig;
use warp::Filter;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "server=debug");
    }
    env_logger::init();

    let config = envy::prefixed("SLACK_").from_env::<SlackConfig>().unwrap();

    let api = filters::root(&config);

    let routes = api.with(warp::log("server"));
    // Start up the server...
    info!("Start up the server");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

mod filters {
    use super::handlers;
    use super::slack::SlackConfig;
    use warp::Filter;

    pub fn root(
        config: &SlackConfig,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        do_not_disturb(config.clone())
            .or(release(config.clone()))
            .or(healthz())
    }

    pub fn do_not_disturb(
        config: SlackConfig,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("do-not-disturb")
            .and(warp::post())
            .and(with_config(config))
            .and_then(handlers::do_not_disturb)
    }

    pub fn release(
        config: SlackConfig,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("release")
            .and(warp::post())
            .and(with_config(config))
            .and_then(handlers::release)
    }

    pub fn healthz() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("healthz")
            .and(warp::get())
            .and_then(handlers::healthz)
    }

    fn with_config(
        config: SlackConfig,
    ) -> impl Filter<Extract = (SlackConfig,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || config.clone())
    }
}

mod handlers {
    use std::convert::Infallible;

    use log::error;
    use warp::http::StatusCode;

    use super::slack::{send_message, SlackConfig};

    pub async fn do_not_disturb(config: SlackConfig) -> Result<impl warp::Reply, Infallible> {
        let msg = format!("<!channel> Do not Disturb :shushing_face: !!!!");
        match send_message(&config, &msg).await {
            Ok(()) => return Ok(StatusCode::OK),
            Err(err) => {
                error!("{}", err);
                return Ok(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
    }

    pub async fn release(config: SlackConfig) -> Result<impl warp::Reply, Infallible> {
        let msg = format!("<!here> Finished :laughing:");
        match send_message(&config, &msg).await {
            Ok(()) => return Ok(StatusCode::OK),
            Err(err) => {
                error!("{}", err);
                return Ok(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
    }

    pub async fn healthz() -> Result<impl warp::Reply, Infallible> {
        Ok(StatusCode::OK)
    }
}

mod slack {
    use anyhow::{anyhow, Error, Result};
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Clone, Deserialize, Debug)]
    pub struct SlackConfig {
        webhook: String,
        username: String,
        channel: String,
        icon_emoji: String,
    }

    pub async fn send_message(config: &SlackConfig, msg: &str) -> Result<(), Error> {
        let mut body = HashMap::new();
        let msg = msg.to_string();
        body.insert("channel", &config.channel);
        body.insert("username", &config.username);
        body.insert("icon_emoji", &config.icon_emoji);
        body.insert("text", &msg);

        let res = reqwest::Client::new()
            .post(&config.webhook)
            .json(&body)
            .send()
            .await
            .map_err(|e| anyhow!(format!("faled to send message: {}", e)))?;

        if res.status() != 200 {
            return Err(anyhow!(format!(
                "faled to send message: code: {}",
                res.status()
            )));
        }
        return Ok(());
    }
}
