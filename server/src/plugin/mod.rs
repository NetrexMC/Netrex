// Home of the plugin api!
pub trait Plugin {
    /// The name of the plugin
    fn get_name() -> String;

    // fn get_config() -> Config;
}
