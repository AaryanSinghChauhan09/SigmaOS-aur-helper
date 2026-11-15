//! AUR RPC API client
//!
//! Handles communication with the Arch User Repository RPC API

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// AUR RPC API endpoint
const AUR_RPC_URL: &str = "https://aur.archlinux.org/rpc";

/// AUR RPC response
#[derive(Debug, Deserialize)]
struct AurResponse<T> {
    #[serde(rename = "type")]
    response_type: String,
    resultcount: u32,
    results: Vec<T>,
    error: Option<String>,
}

/// AUR package information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AurPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: Option<String>,
    pub maintainer: Option<String>,
    pub depends: Option<Vec<String>>,
    pub makedepends: Option<Vec<String>>,
    pub optdepends: Option<Vec<String>>,
    pub keywords: Option<Vec<String>>,
    pub popularity: f64,
}

/// AUR RPC client
pub struct AurClient {
    client: Client,
}

impl AurClient {
    /// Create a new AUR client
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Search for packages
    pub async fn search(&self, query: &str) -> Result<Vec<AurPackage>> {
        let url = format!(
            "{}?v=5&type=search&arg={}",
            AUR_RPC_URL,
            urlencoding::encode(query)
        );

        let response: AurResponse<AurPackage> = self.client.get(&url).send().await?.json().await?;

        if response.response_type == "error" {
            anyhow::bail!("AUR RPC error: {:?}", response.error);
        }

        Ok(response.results)
    }

    /// Get package information by name
    pub async fn info(&self, name: &str) -> Result<Option<AurPackage>> {
        let url = format!(
            "{}?v=5&type=info&arg={}",
            AUR_RPC_URL,
            urlencoding::encode(name)
        );

        let response: AurResponse<AurPackage> = self.client.get(&url).send().await?.json().await?;

        if response.response_type == "error" {
            anyhow::bail!("AUR RPC error: {:?}", response.error);
        }

        Ok(response.results.into_iter().next())
    }

    /// Get multiple packages by names
    pub async fn multiinfo(&self, names: &[String]) -> Result<Vec<AurPackage>> {
        if names.is_empty() {
            return Ok(vec![]);
        }

        let args = names.join("&arg[]=");
        let url = format!(
            "{}?v=5&type=multiinfo&arg[]={}",
            AUR_RPC_URL,
            urlencoding::encode(&args)
        );

        let response: AurResponse<AurPackage> = self.client.get(&url).send().await?.json().await?;

        if response.response_type == "error" {
            anyhow::bail!("AUR RPC error: {:?}", response.error);
        }

        Ok(response.results)
    }
}

impl Default for AurClient {
    fn default() -> Self {
        Self::new()
    }
}
