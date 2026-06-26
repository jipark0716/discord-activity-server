use anyhow::Context;
use serde::Deserialize;

pub struct DiscordClient {
    http: reqwest::Client,
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct DiscordTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
}

impl DiscordClient {
    pub fn new(client_id: String, client_secret: String) -> anyhow::Result<Self> {
        let http = reqwest::Client::builder()
            .pool_max_idle_per_host(8)
            .build()
            .context("failed to build HTTP client")?;

        Ok(Self {
            http,
            client_id,
            client_secret,
        })
    }

    pub async fn authorization(&self, code: &str) -> anyhow::Result<DiscordTokenResponse> {
        let response = self
            .http
            .post("https://discord.com/api/oauth2/token")
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("grant_type", "authorization_code"),
                ("code", code),
            ])
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();

            tracing::info!(
                status = %status,
                body = %body,
                "Discord authorization failed"
            );

            anyhow::bail!("Discord authorization failed: {status}");
        }

        Ok(response.json().await?)
    }
}
