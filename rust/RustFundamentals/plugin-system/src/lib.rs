pub trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self);
}

pub struct PluginManager {
    pub plugins: Vec<Box<dyn Plugin>>,
}

// 3. Implement the PluginManager
impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        if let Some(_) = self.plugins.iter().find(|p| p.name() == plugin.name()) {
            panic!("Plugin with name {} already exists", plugin.name());
        } else {
            self.plugins.push(plugin)
        };
    }

    pub fn remove_plugin(&mut self, name: &str) -> Option<Box<dyn Plugin>> {
        if let Some(pos) = self.plugins.iter().position(|p| p.name() == name) {
            Some(self.plugins.remove(pos))
        } else {
            None
        }
    }

    pub fn execute_all(&self) {
        for plugin in &self.plugins {
            plugin.execute();
        }
    }
}

// Example usage
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "MyPlugin"
    }
    fn execute(&self) {
        println!("Executing MyPlugin");
    }
}

impl MyPlugin {
    fn new() -> Self {
        Self
    }
}

pub fn main() {
    let mut manager = PluginManager::new();

    manager.add_plugin(Box::new(MyPlugin::new()));
    manager.execute_all();
}
