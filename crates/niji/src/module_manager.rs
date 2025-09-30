use std::{collections::HashSet, path::PathBuf, rc::Rc, sync::Mutex};

use anyhow::{anyhow, Context};
use log::{debug, error, info};
use niji_console::heading;

use crate::{
	config::{Config, Theme},
	file_manager::FileManager,
	files::Files,
	lua::runtime::{LuaRuntime, LuaRuntimeInit},
	module::Module,
	utils::xdg::XdgDirs,
};

pub struct ModuleManagerInit {
	pub xdg: Rc<XdgDirs>,
	pub files: Rc<Files>,
	pub config: Rc<Config>,
	pub file_manager: Rc<FileManager>,
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
		ModuleManagerInit {
			xdg,
			files,
			config,
			file_manager,
		}: ModuleManagerInit,
	) -> anyhow::Result<Self> {
		let mut active_modules = Vec::<ModuleDescriptor>::with_capacity(config.modules.len());
		for mod_name in &config.modules {
			Self::activate(&files, &mut active_modules, mod_name)?;
		}

		let lua_runtime = LuaRuntime::new(LuaRuntimeInit {
			xdg: Rc::clone(&xdg),
			files: Rc::clone(&files),
			file_manager: Rc::clone(&file_manager),
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
		reload: bool,
		modules: Option<&[String]>,
	) -> anyhow::Result<()> {
		let mut remaining = HashSet::<String>::new();
		if let Some(modules) = modules {
			remaining.extend(modules.iter().cloned())
		}

		for module_descr in &*self.active_modules.lock().unwrap() {
			if modules.is_some() && !remaining.remove(&module_descr.name.clone()) {
				continue;
			}

			self.apply_module(module_descr, config, theme, reload);
		}

		if modules.is_some() {
			for mod_name in remaining {
				let module_descr = Self::activate(
					&self.files,
					&mut self.active_modules.lock().unwrap(),
					&mod_name,
				)?;
				self.apply_module(&module_descr, config, theme, reload);
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
			.ok_or_else(|| anyhow!("Module \"{}\" does not exist", mod_name))?;

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
		reload: bool,
	) {
		heading!("{}", module_descr.name);

		let module = match Module::load(&self.lua_runtime, &module_descr.path) {
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

		if let Err(err) = module.apply(module_config.clone(), theme.clone()) {
			error!("{err:?}");
			error!("Aborting module execution");
			niji_console::println!();
			return;
		}
		if reload {
			if config.disable_reloads.is_disabled(&module_descr.name) {
				info!(
					"Reloading is disabled for module {}. You will only see the changes after a \
					 restart",
					module_descr.name
				)
			} else if module.can_reload() {
				info!("Reloading...");
				if let Err(err) = module.reload(module_config) {
					error!("{err:?}");
					error!("Reloading of {} failed", module_descr.name);
					niji_console::println!();
				}
			} else {
				debug!("Module {} does not support reloading.", module_descr.name)
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
