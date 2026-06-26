use anyhow::Context;

#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) discord_client_id: String,
    pub(crate) discord_secret_key: String,
}

macro_rules! env_var {
    ($name:ident) => {
        std::env::var(stringify!($name))
            .with_context(|| concat!(stringify!($name), " is not set"))?
    };
}

pub(crate) fn load_from_env() -> anyhow::Result<Config> {
    Ok(Config {
        discord_client_id: env_var!(DISCORD_CLIENT_ID),
        discord_secret_key: env_var!(DISCORD_SECRET_KEY),
    })
}