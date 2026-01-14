use std::{collections::HashMap, fs, path::Path};

use niji_macros::IntoLua;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, IntoLua, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModuleConfigValue {
	#[default]
	Nil,
	String(String),
	Int(i64),
	Float(f64),
	Bool(bool),
	Vec(Vec<ModuleConfigValue>),
	Map(HashMap<String, ModuleConfigValue>),
}

pub type ModuleConfig = HashMap<String, ModuleConfigValue>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DisableReloads {
	#[default]
	None,

	All,

	#[serde(untagged)]
	Blacklist(Vec<String>),
}

impl DisableReloads {
	pub fn is_disabled(&self, name: &str) -> bool {
		match self {
			Self::None => false,
			Self::All => true,
			Self::Blacklist(blacklist) => blacklist.contains(&name.to_string()),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	pub modules: Vec<String>,

	#[serde(default)]
	pub disable_reloads: DisableReloads,

	#[serde(default)]
	pub global: ModuleConfig,

	#[serde(flatten)]
	#[allow(clippy::struct_field_names)]
	pub module_config: HashMap<String, ModuleConfig>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			modules: vec![],
			disable_reloads: DisableReloads::None,
			global: ModuleConfig::new(),
			module_config: HashMap::new(),
		}
	}
}

pub fn read_config(path: impl AsRef<Path>) -> anyhow::Result<Config> {
	if !path.as_ref().exists() {
		return Ok(Config::default());
	}
	let config_str = fs::read_to_string(&path)?;
	let config: Config = toml::from_str(&config_str)?;
	Ok(config)
}
