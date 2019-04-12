
use cmdr::*;

struct CaveScope {}

#[cmdr]
impl CaveScope {
    #[cmd]
    fn north(&self, _args: &[String]) -> CommandResult {
        println!("You walk north");
        CommandResult::Ok
    }

    #[cmd]
    fn south(&self, _args: &[String]) -> CommandResult {
        println!("You walk south");
        CommandResult::Ok
    }
    
    #[cmd]
    fn east(&self, _args: &[String]) -> CommandResult {
        println!("You walk east");
        CommandResult::Ok
    }

    #[cmd]
    fn west(&self, _args: &[String]) -> CommandResult {
        println!("You walk west");
        CommandResult::Ok
    }
}

fn main() {
    cmd_loop(&mut CaveScope {})
}
