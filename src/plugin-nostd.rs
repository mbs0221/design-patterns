pub trait Plugin {
    fn perform_action(&self);
}

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn perform_action(&self) {
        println!("Performing action from MyPlugin");
    }
}

// 定义函数指针类型
type PluginFn = fn() -> Box<dyn Plugin>;

// 创建函数用于返回具体插件实例的函数指针
fn create_my_plugin() -> Box<dyn Plugin> {
    Box::new(MyPlugin)
}

use ahash::AHashMap as HashMap;

pub struct PluginManager {
    plugins: HashMap<String, PluginFn>,
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: HashMap::new(),
        }
    }

    pub fn load_plugin(&mut self, name: String, create_fn: PluginFn) {
        self.plugins.insert(name, create_fn);
    }

    pub fn run_plugin_action(&self, name: &str) {
        if let Some(create_fn) = self.plugins.get(name) {
            let plugin = create_fn();
            plugin.perform_action();
        }
    }
}

fn main() {
    let mut plugin_manager = PluginManager::new();

    // 加载插件
    plugin_manager.load_plugin(String::from("my_plugin"), create_my_plugin);

    // 执行插件操作
    plugin_manager.run_plugin_action("my_plugin");
}
