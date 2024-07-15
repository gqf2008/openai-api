use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod audio;
pub mod chat;
pub mod completions;
pub mod embeddings;
pub mod images;
pub mod models;

// Models API
const MODELS_LIST: &str = "models";
const MODELS_RETRIEVE: &str = "models/";
// Completions API
const COMPLETION_CREATE: &str = "completions";
// Chat API
const CHAT_COMPLETION_CREATE: &str = "chat/completions";
// Images API
const IMAGES_CREATE: &str = "images/generations";
const IMAGES_EDIT: &str = "images/edits";
const IMAGES_VARIATIONS: &str = "images/variations";
// Embeddings API
const EMBEDDINGS_CREATE: &str = "embeddings";
// Audio API
const AUDIO_TRANSCRIPTION_CREATE: &str = "audio/transcriptions";
const AUDIO_TRANSLATIONS_CREATE: &str = "audio/translations";

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
	pub prompt_tokens: Option<u32>,
	pub completion_tokens: Option<u32>,
	pub total_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
	pub text: Option<String>,
	pub index: u32,
	pub logprobs: Option<String>,
	pub finish_reason: Option<String>,
	pub message: Option<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
	pub role: Role,
	pub content: String,
}

impl Clone for Message {
	fn clone(&self) -> Self {
		Self { role: self.role.clone(), content: self.content.clone() }
	}
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
	pub role: Role,
	pub content: Vec<Content>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
	#[serde(rename = "type")]
	pub type_: String,
	#[serde(flatten)]
	content: serde_json::Value,
}

impl Content {
	pub fn with_text<S: ToString>(txt: S) -> Self {
		let content = serde_json::json!({ "text": {"url": txt.to_string()} });
		Self { type_: "text".to_string(), content }
	}

	pub fn with_image<S: ToString>(url: S) -> Self {
		let base64 = url.to_string();
		let content = serde_json::json!({ "image_url": {"url":base64} });

		Self { type_: "text".to_string(), content }
	}

	pub fn with_jpeg(bytes: &[u8]) -> Self {
		let base64 = format!("data:image/jpeg;base64,{}", base64::encode(bytes));
		let content = serde_json::json!({ "image_url": {"url":base64} });

		Self { type_: "text".to_string(), content }
	}

	pub fn with_png(bytes: &[u8]) -> Self {
		let base64 = format!("data:image/png;base64,{}", base64::encode(bytes));
		let content = serde_json::json!({ "image_url": {"url":base64} });
		Self { type_: "text".to_string(), content }
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
	System,
	Assistant,
	User,
}

impl Clone for Role {
	fn clone(&self) -> Self {
		match self {
			Self::System => Self::System,
			Self::Assistant => Self::Assistant,
			Self::User => Self::User,
		}
	}
}
