use std::{collections::HashSet, path::PathBuf, rc::Rc, sync::Mutex};

use anyhow::{Context, anyhow};
use log::{debug, error, info};
use niji_console::heading;

use crate::{
	config::Config,
	files::Files,
	lua::runtime::{LuaRuntime, LuaRuntimeInit},
	module::Module,
	theme::Theme,
	types::color::Color,
	utils::xdg::XdgDirs,
};

pub struct ModuleManagerInit {
	pub xdg: Rc<XdgDirs>,
	pub files: Rc<Files>,
	pub config: Rc<Config>,
}

#[derive(Debug, Clone)]
pub struct ApplyParams {
	pub reload: bool,
	pub check_deps: bool,
}

#[derive(Clone)]
struct ModuleDescriptor {
	name: String,
	path: PathBuf,
}

pub struct ModuleManager {
	files: Rc<Files>,
	active_modules: Mutex<Vec<ModuleDescriptor>>,
	lua_runtime: LuaRuntime,
}

impl ModuleManager {
	pub fn new(
		ModuleManagerInit { xdg, files, config }: ModuleManagerInit,
	) -> anyhow::Result<Self> {
		let mut active_modules = Vec::<ModuleDescriptor>::with_capacity(config.modules.len());
		for mod_name in &config.modules {
			Self::activate(&files, &mut active_modules, mod_name)?;
		}

		let lua_runtime = LuaRuntime::new(LuaRuntimeInit {
			xdg: Rc::clone(&xdg),
			files: Rc::clone(&files),
		})
		.context("Failed to initialize lua runtime")?;

		Ok(Self {
			files: Rc::clone(&files),
			active_modules: Mutex::new(active_modules),
			lua_runtime,
		})
	}

	pub fn apply(
		&self,
		config: &Config,
		theme: &Theme,
		accent: Color,
		params: &ApplyParams,
		modules: Option<&[String]>,
	) -> anyhow::Result<()> {
		let mut remaining = HashSet::<String>::new();
		if let Some(modules) = modules {
			remaining.extend(modules.iter().cloned());
		}

		for module_descr in &*self.active_modules.lock().unwrap() {
			if modules.is_some() && !remaining.remove(&module_descr.name.clone()) {
				continue;
			}

			self.apply_module(module_descr, config, theme, accent, params);
		}

		if modules.is_some() {
			for mod_name in remaining {
				let module_descr = Self::activate(
					&self.files,
					&mut self.active_modules.lock().unwrap(),
					&mod_name,
				)?;
				self.apply_module(&module_descr, config, theme, accent, params);
			}
		}

		Ok(())
	}

	fn activate(
		files: &Files,
		active_modules: &mut Vec<ModuleDescriptor>,
		mod_name: &str,
	) -> anyhow::Result<ModuleDescriptor> {
		let module_dir = Self::find_module_dir(files, mod_name)
			.ok_or_else(|| anyhow!("Module \"{mod_name}\" does not exist"))?;

		debug!(
			"Activating module \"{mod_name}\" at path {}",
			module_dir.display()
		);

		let module_descr = ModuleDescriptor {
			name: mod_name.to_string(),
			path: module_dir,
		};

		active_modules.push(module_descr.clone());

		Ok(module_descr)
	}

	fn apply_module(
		&self,
		module_descr: &ModuleDescriptor,
		config: &Config,
		theme: &Theme,
		accent: Color,
		params: &ApplyParams,
	) {
		heading!("{}", module_descr.name);

		let module = match Module::load(&self.lua_runtime, &module_descr.path, params.check_deps) {
			Ok(module) => module,
			Err(error) => {
				error!("{error:?}");
				niji_console::println!();
				return;
			}
		};

		let mut module_config = config.global.clone();
		if let Some(specific) = config.module_config.get(&module_descr.name) {
			module_config.extend(specific.clone());
		}

		if let Err(err) = module.apply(module_config.clone(), theme.clone(), accent) {
			error!("{err:?}");
			error!("Aborting module execution");
			niji_console::println!();
			return;
		}
		if params.reload {
			if config.disable_reloads.is_disabled(&module_descr.name) {
				info!(
					"Reloading is disabled for module {}. You will only see the changes after a \
					 restart",
					module_descr.name
				);
			} else if module.can_reload() {
				info!("Reloading...");
				if let Err(err) = module.reload(module_config) {
					error!("{err:?}");
					error!("Reloading of {} failed", module_descr.name);
					niji_console::println!();
				}
			} else {
				debug!("Module {} does not support reloading.", module_descr.name);
			}
		}
		info!("Done!");
		niji_console::println!();
	}

	fn find_module_dir(files: &Files, name: &str) -> Option<PathBuf> {
		for module_location in files.iter_modules() {
			if module_location.name == name {
				return Some(module_location.path);
			}
		}
		None
	}
}

#[cfg(test)]
mod tests {
	use std::{collections::HashMap, fs};

	use tempfile::tempdir;

	use crate::{config::DisableReloads, theme::test_utils::test_theme};

	use super::*;

	#[test]
	fn init() {
		let tempdir = tempdir().unwrap();
		let xdg = Rc::new(XdgDirs::in_tempdir(&tempdir));
		let files = Rc::new(Files::new(&xdg).unwrap());
		let config = Rc::new(Config {
			modules: vec![],
			disable_reloads: DisableReloads::None,
			global: HashMap::new(),
			module_config: HashMap::new(),
		});
		ModuleManager::new(ModuleManagerInit { xdg, files, config }).unwrap();
	}

	#[test]
	fn apply_module() {
		let tempdir = tempdir().unwrap();
		let xdg = Rc::new(XdgDirs::in_tempdir(&tempdir));
		let files = Rc::new(Files::new(&xdg).unwrap());
		let config = Rc::new(Config {
			modules: vec![],
			disable_reloads: DisableReloads::None,
			global: HashMap::new(),
			module_config: HashMap::new(),
		});
		let module_manager = ModuleManager::new(ModuleManagerInit {
			xdg: xdg.clone(),
			files,
			config: config.clone(),
		})
		.unwrap();

		fs::create_dir_all(xdg.config_home.join("niji/modules/test")).unwrap();
		fs::write(
			xdg.config_home.join("niji/modules/test/module.lua"),
			"function apply(config, theme) end",
		)
		.unwrap();

		module_manager
			.apply(
				&config,
				&test_theme(),
				Color::BLACK,
				&ApplyParams {
					reload: false,
					check_deps: true,
				},
				Some(&["test".to_string()]),
			)
			.unwrap();
	}

	#[test]
	fn apply_module_error() {
		let tempdir = tempdir().unwrap();
		let xdg = Rc::new(XdgDirs::in_tempdir(&tempdir));
		let files = Rc::new(Files::new(&xdg).unwrap());
		let config = Rc::new(Config {
			modules: vec![],
			disable_reloads: DisableReloads::None,
			global: HashMap::new(),
			module_config: HashMap::new(),
		});
		let module_manager = ModuleManager::new(ModuleManagerInit {
			xdg: xdg.clone(),
			files,
			config: config.clone(),
		})
		.unwrap();

		fs::create_dir_all(xdg.config_home.join("niji/modules/test")).unwrap();
		fs::write(
			xdg.config_home.join("niji/modules/test/module.lua"),
			"this is a syntax error! yay :3",
		)
		.unwrap();

		// This should not error, instead there should be a log message
		module_manager
			.apply(
				&config,
				&test_theme(),
				Color::BLACK,
				&ApplyParams {
					reload: false,
					check_deps: true,
				},
				Some(&["test".to_string()]),
			)
			.unwrap();
	}
}
