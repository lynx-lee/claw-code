use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

use serde::Deserialize;

use crate::error::ApiError;

static PROVIDER_REGISTRY: OnceLock<HashMap<String, ProviderConfig>> = OnceLock::new();
static MODEL_REGISTRY: OnceLock<HashMap<String, ModelConfig>> = OnceLock::new();

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderConfigFile {
    pub providers: Vec<ProviderConfig>,
    pub models: Vec<ModelConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub api_type: ApiType,
    pub api_key_env: String,
    pub base_url_env: Option<String>,
    pub default_base_url: String,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiType {
    Anthropic,
    OpenAiCompat,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelConfig {
    pub alias: String,
    pub canonical: String,
    pub provider: String,
    pub max_output_tokens: u32,
    pub context_window_tokens: u32,
}

fn default_providers() -> Vec<ProviderConfig> {
    vec![
        ProviderConfig {
            name: "anthropic".to_string(),
            api_type: ApiType::Anthropic,
            api_key_env: "ANTHROPIC_API_KEY".to_string(),
            base_url_env: Some("ANTHROPIC_BASE_URL".to_string()),
            default_base_url: "https://api.anthropic.com".to_string(),
            api_key: None,
            base_url: None,
        },
        ProviderConfig {
            name: "openai".to_string(),
            api_type: ApiType::OpenAiCompat,
            api_key_env: "OPENAI_API_KEY".to_string(),
            base_url_env: Some("OPENAI_BASE_URL".to_string()),
            default_base_url: "https://api.openai.com/v1".to_string(),
            api_key: None,
            base_url: None,
        },
        ProviderConfig {
            name: "xai".to_string(),
            api_type: ApiType::OpenAiCompat,
            api_key_env: "XAI_API_KEY".to_string(),
            base_url_env: Some("XAI_BASE_URL".to_string()),
            default_base_url: "https://api.x.ai/v1".to_string(),
            api_key: None,
            base_url: None,
        },
        ProviderConfig {
            name: "deepseek".to_string(),
            api_type: ApiType::OpenAiCompat,
            api_key_env: "DEEPSEEK_API_KEY".to_string(),
            base_url_env: Some("DEEPSEEK_BASE_URL".to_string()),
            default_base_url: "https://api.deepseek.com/v1".to_string(),
            api_key: None,
            base_url: None,
        },
        ProviderConfig {
            name: "ollama".to_string(),
            api_type: ApiType::OpenAiCompat,
            api_key_env: "OLLAMA_API_KEY".to_string(),
            base_url_env: Some("OLLAMA_BASE_URL".to_string()),
            default_base_url: "http://localhost:11434/v1".to_string(),
            api_key: None,
            base_url: None,
        },
    ]
}

fn default_models() -> Vec<ModelConfig> {
    vec![
        ModelConfig {
            alias: "opus".to_string(),
            canonical: "claude-opus-4-6".to_string(),
            provider: "anthropic".to_string(),
            max_output_tokens: 32_000,
            context_window_tokens: 200_000,
        },
        ModelConfig {
            alias: "sonnet".to_string(),
            canonical: "claude-sonnet-4-6".to_string(),
            provider: "anthropic".to_string(),
            max_output_tokens: 64_000,
            context_window_tokens: 200_000,
        },
        ModelConfig {
            alias: "haiku".to_string(),
            canonical: "claude-haiku-4-5-20251213".to_string(),
            provider: "anthropic".to_string(),
            max_output_tokens: 64_000,
            context_window_tokens: 200_000,
        },
        ModelConfig {
            alias: "grok".to_string(),
            canonical: "grok-3".to_string(),
            provider: "xai".to_string(),
            max_output_tokens: 64_000,
            context_window_tokens: 131_072,
        },
        ModelConfig {
            alias: "grok-3".to_string(),
            canonical: "grok-3".to_string(),
            provider: "xai".to_string(),
            max_output_tokens: 64_000,
            context_window_tokens: 131_072,
        },
        ModelConfig {
            alias: "grok-mini".to_string(),
            canonical: "grok-3-mini".to_string(),
            provider: "xai".to_string(),
            max_output_tokens: 64_000,
            context_window_tokens: 131_072,
        },
        ModelConfig {
            alias: "grok-3-mini".to_string(),
            canonical: "grok-3-mini".to_string(),
            provider: "xai".to_string(),
            max_output_tokens: 64_000,
            context_window_tokens: 131_072,
        },
        ModelConfig {
            alias: "grok-2".to_string(),
            canonical: "grok-2".to_string(),
            provider: "xai".to_string(),
            max_output_tokens: 64_000,
            context_window_tokens: 131_072,
        },
        ModelConfig {
            alias: "deepseek".to_string(),
            canonical: "deepseek-chat".to_string(),
            provider: "deepseek".to_string(),
            max_output_tokens: 8_192,
            context_window_tokens: 64_000,
        },
        ModelConfig {
            alias: "llama3".to_string(),
            canonical: "llama3".to_string(),
            provider: "ollama".to_string(),
            max_output_tokens: 4_096,
            context_window_tokens: 128_000,
        },
        ModelConfig {
            alias: "llama3.1".to_string(),
            canonical: "llama3.1".to_string(),
            provider: "ollama".to_string(),
            max_output_tokens: 4_096,
            context_window_tokens: 128_000,
        },
        ModelConfig {
            alias: "qwen".to_string(),
            canonical: "qwen2.5".to_string(),
            provider: "ollama".to_string(),
            max_output_tokens: 4_096,
            context_window_tokens: 128_000,
        },
    ]
}

fn config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("claw-code").join("providers.toml"))
}

fn load_config_from_file() -> Option<ProviderConfigFile> {
    let path = config_path()?;
    if !path.exists() {
        return None;
    }
    let content = std::fs::read_to_string(&path).ok()?;
    toml::from_str(&content).ok()
}

pub fn init_registry() -> Result<(), ApiError> {
    let file_config = load_config_from_file();

    let providers: HashMap<String, ProviderConfig> = file_config.as_ref().map_or_else(
        || {
            default_providers()
                .into_iter()
                .map(|p| (p.name.clone(), p))
                .collect()
        },
        |c| {
            c.providers
                .iter()
                .map(|p| (p.name.clone(), p.clone()))
                .collect()
        },
    );

    let models: HashMap<String, ModelConfig> = file_config.as_ref().map_or_else(
        || {
            default_models()
                .into_iter()
                .map(|m| (m.alias.clone(), m))
                .collect()
        },
        |c| {
            c.models
                .iter()
                .map(|m| (m.alias.clone(), m.clone()))
                .collect()
        },
    );

    let _ = PROVIDER_REGISTRY.set(providers);
    let _ = MODEL_REGISTRY.set(models);

    Ok(())
}

fn ensure_initialized() {
    if PROVIDER_REGISTRY.get().is_none() {
        let _ = init_registry();
    }
}

#[must_use]
pub fn get_provider_config(name: &str) -> Option<ProviderConfig> {
    ensure_initialized();
    PROVIDER_REGISTRY.get()?.get(name).cloned()
}

#[must_use]
pub fn get_model_config(alias: &str) -> Option<ModelConfig> {
    ensure_initialized();
    MODEL_REGISTRY.get()?.get(alias).cloned()
}

#[must_use]
pub fn all_providers() -> Vec<ProviderConfig> {
    ensure_initialized();
    PROVIDER_REGISTRY
        .get()
        .map(|r| r.values().cloned().collect())
        .unwrap_or_default()
}

#[must_use]
pub fn all_models() -> Vec<ModelConfig> {
    ensure_initialized();
    MODEL_REGISTRY
        .get()
        .map(|r| r.values().cloned().collect())
        .unwrap_or_default()
}

impl ProviderConfig {
    pub fn get_api_key(&self) -> Option<String> {
        if let Some(key) = &self.api_key {
            if !key.is_empty() {
                return Some(key.clone());
            }
        }
        std::env::var(&self.api_key_env).ok().filter(|s| !s.is_empty())
    }

    pub fn get_base_url(&self) -> String {
        if let Some(url) = &self.base_url {
            if !url.is_empty() {
                return url.clone();
            }
        }
        if let Some(env_var) = &self.base_url_env {
            if let Ok(url) = std::env::var(env_var) {
                if !url.is_empty() {
                    return url;
                }
            }
        }
        self.default_base_url.clone()
    }

    pub fn has_api_key(&self) -> bool {
        self.get_api_key().is_some()
    }
}
