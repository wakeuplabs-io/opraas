use opraas_core::config::{CoreConfig, TreeConfig};

pub struct Config {
    pub tree: TreeConfig,
    pub core: Option<CoreConfig>,
}

impl Config {
    pub fn new_from_root<P: AsRef<std::path::Path>>(
        root: &P,
    ) -> Self {
        let tree = TreeConfig::new_from_root(root.as_ref().to_path_buf());
        let core = CoreConfig::new_from_toml(&root.as_ref().join("config.toml")).ok();

        Self { tree, core }
    }

    pub fn build_core(&self) -> Result<opraas_core::config::Config, Box<dyn std::error::Error>> {
        if let Some(core) = &Self::new_from_root(&std::env::current_dir()?).core {
            Ok(opraas_core::config::Config {
                tree: self.tree.clone(),
                core: core.clone(),
            })
        } else {
            Err("Core config not found. Create project with opraas new".into())
        }
    }

}
