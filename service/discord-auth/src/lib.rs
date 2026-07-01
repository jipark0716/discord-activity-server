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

#[derive(Debug, Deserialize, Clone)]
pub struct DiscordUserResponse {
    pub id: u64,
    pub username: String,
    pub avatar: String,
}

impl DiscordUserResponse {
    pub fn profile(&self, size: u16) -> String {
        let Self {
            id,
            avatar,
            ..
        } = self;

        format!("https://cdn.discordapp.com/avatars/{id}/{avatar}.webp?size={size}")
    }
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

    pub async fn get_current_user(&self, access_token: &str) -> anyhow::Result<DiscordUserResponse> {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert("Authorization", format!("Bearer {access_token}").parse()?);

        let response = self.http
          .request(reqwest::Method::GET, "https://discord.com/api/v10/users/@me")
          .headers(headers)
          .send()
          .await?;

        let status = response.status();

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();

            tracing::info!(
                status = %status,
                body = %body,
                "discord get user fail"
            );

            anyhow::bail!("Discord authorization failed: {status}");
        }

        Ok(response.json().await?)
    }
}
