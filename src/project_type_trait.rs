use std::fmt::Debug;

pub trait ProjectTypeCommands: Debug {
    #[allow(dead_code)]
    fn name(&self) -> &'static str;
    fn install_command(&self) -> String;
    fn migrate_command(&self) -> Option<String> {
        None
    }
    fn console_command(&self) -> Option<String> {
        None
    }
    fn start_command(&self) -> Option<String> {
        None
    }
    fn test_command(&self) -> Option<String> {
        None
    }
    fn routes_command(&self) -> Option<String> {
        None
    }
}
