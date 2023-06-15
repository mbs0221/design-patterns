pub trait Plugin {
    fn perform_action(&self);
}

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn perform_action(&self) {
        println!("Performing action from MyPlugin");
    }
}

pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: HashMap::new(),
        }
    }

    pub fn load_plugin(&mut self, name: String, plugin: Box<dyn Plugin>) {
        self.plugins.insert(name, plugin);
    }

    pub fn run_plugin_action(&self, name: &str) {
        if let Some(plugin) = self.plugins.get(name) {
            plugin.perform_action();
        }
    }
}

fn main() {
    let mut plugin_manager = PluginManager::new();

    // 加载插件
    let my_plugin = Box::new(MyPlugin);
    plugin_manager.load_plugin(String::from("my_plugin"), my_plugin);

    // 执行插件操作
    plugin_manager.run_plugin_action("my_plugin");
}
