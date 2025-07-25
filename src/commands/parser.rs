use colored::*;

#[derive(Debug, Clone)]
pub enum Command {
    Help,
    Ping,
    Quit,
    Leave,
    Status,
    DM { recipient: String, message: String },

    Account,
    AccountRegister { username: String, password: String, confirm: String },
    AccountLogin { username: String, password: String },
    AccountLogout,
    AccountEditUsername { username: String },
    AccountEditPassword { current_password: String, new_password: String },
    AccountImport { filename: String },
    AccountExport { filename: String },
    AccountDelete { force: bool },

    RoomList,
    RoomCreate { name: String, whitelist: bool },
    RoomJoin { name: String },
    RoomImport { filename: String },
    RoomDelete { name: String, force: bool },
    InvalidSyntax { err_msg: String },
    Unavailable
}

pub fn parse_command(input: &str) -> Command {
    let mut tokens: Vec<&str> = input.trim().split_whitespace().collect();
    tokens[0] = &tokens[0][1..];

    match tokens.as_slice() {
        ["help"] | ["h"] => Command::Help,
        ["ping"] => Command::Ping,
        ["quit"] | ["exit"] | ["q"] | ["e"] => Command::Quit,
        ["leave"] => Command::Leave,
        ["status"] => Command::Status,
        
        ["message", recipient, message] |
        ["msg", recipient, message] |
        ["dm", recipient, message] => Command::DM {
            recipient: recipient.to_string(),
            message: message.to_string()
        },

        ["message", ..] |
        ["msg", ..] |
        ["dm", ..]=> {
            let err_msg = format!("{}", "Usage: /message <recipient> <message>".bright_blue());
            Command::InvalidSyntax { err_msg }
        },

        ["account", "register", username, password, confirm_password] |
        ["a", "register", username, password, confirm_password] |
        ["account", "r", username, password, confirm_password] |
        ["a", "r", username, password, confirm_password] => Command::AccountRegister {
            username: username.to_string(),
            password: password.to_string(),
            confirm: confirm_password.to_string()
        },

        ["account", "register", ..] |
        ["a", "register", ..] |
        ["account", "r", ..] |
        ["a", "r", ..] => {
            let err_msg = format!("{}", "Usage: /account register <username> <password> <password confirm>".bright_blue());
            Command::InvalidSyntax { err_msg }
        },

        ["account", "login", u, p] |
        ["a", "login", u, p] |
        ["account", "l", u, p] |
        ["a", "l", u, p] => Command::AccountLogin {
            username: u.to_string(),
            password: p.to_string(),
        },

        ["account", "login", ..] |
        ["a", "login", ..] |
        ["account", "l", ..] |
        ["a", "l", ..] => {
            let err_msg = format!("{}", "Usage: /account login <username> <password>".bright_blue());
            Command::InvalidSyntax { err_msg }
        },

        ["account", "logout"] |
        ["a", "logout"] => Command::AccountLogout {},

        ["account", "logout", ..] |
        ["a", "logout", ..] => {
            let err_msg = format!("{}", "Usage: /account logout".bright_blue());
            Command::InvalidSyntax { err_msg }
        },

        ["account", "edit", "username", username] |
        ["a", "edit", "username", username] |
        ["account", "e", "username", username] |
        ["account", "edit", "u", username] |
        ["a", "e", "username", username] |
        ["a", "edit", "u", username] |
        ["account", "e", "u", username] |
        ["a", "e", "u", username] => Command::AccountEditUsername {
            username: username.to_string()
        },

        ["account", "edit", "password", current_password, new_password] |
        ["a", "edit", "password", current_password, new_password] |
        ["account", "e", "password", current_password, new_password] |
        ["account", "edit", "p", current_password, new_password] |
        ["a", "e", "password", current_password, new_password] |
        ["a", "edit", "p", current_password, new_password] |
        ["account", "e", "p", current_password, new_password] |
        ["a", "e", "p", current_password, new_password] => Command::AccountEditPassword {
            current_password: current_password.to_string(),
            new_password: new_password.to_string()
        },

        ["account", "edit", ..] |
        ["a", "edit", ..] |
        ["account", "e", ..] |
        ["a", "e", ..] => {
            let err_msg = format!("{}", "Usage: /account edit username <new username> or /account edit password <current password> <new password>".bright_blue());
            Command::InvalidSyntax { err_msg }
        },

        ["account", "import", filename] |
        ["a", "import", filename] => Command::AccountImport {
            filename: filename.to_string()
        },

        ["account", "import", ..] |
        ["a", "import", ..] => {
            let err_msg = format!("{}", "Usage: /account import <filename>".bright_blue());
            Command::InvalidSyntax { err_msg }
        },

        ["account", "export"] |
        ["a", "export"] => Command::AccountExport {
            filename: "".to_string()
        },

        ["account", "export", filename] |
        ["a", "export", filename] => Command::AccountExport {
            filename: filename.to_string()
        },

        ["account", "export", ..] |
        ["a", "export", ..] => {
            let err_msg = format!("{}", "Usage: /account export or /account export <filename>".bright_blue());
            Command::InvalidSyntax { err_msg }
        },

        ["account", "delete"] |
        ["a", "delete"] | 
        ["account", "d"] |
        ["a", "d"] => Command::AccountDelete { force: false },

        ["account", "delete", "force"] |
        ["a", "delete", "force"] |
        ["account", "d", "force"] |
        ["account", "delete", "f"] |
        ["a", "d", "force"] |
        ["a", "delete", "f"] |
        ["account", "d", "f"] |
        ["a", "d", "f"] => Command::AccountDelete { force: true },

        ["account", "delete", ..] |
        ["a", "delete", ..] |
        ["account", "d", ..] |
        ["a", "d", ..] => {
            let err_msg = format!("{}", "Usage: /account delete or /account delete force".bright_blue());
            Command::InvalidSyntax { err_msg }
        },

        ["account", "info"] |
        ["a", "info"] |
        ["account", "i"] |
        ["a", "i"] => Command::Account,

        ["account", "info", ..] |
        ["a", "info", ..] |
        ["account", "i", ..] |
        ["a", "i", ..] => {
            let err_msg = format!("{}", "Usage: /account info".bright_blue());
            Command::InvalidSyntax { err_msg }
        }

        ["account", ..] |
        ["a", ..] => {
            let err_msg = format!("{}", "Account commands:\n> /account info\n> /account register <username> <password> <password confirm>\n> /account login <username> <password>\n> /account logout\n> /account edit username <new username> or /account edit password <current password> <new password>\n> /account import <filename>\n> /account export or /account export <filename>\n> /account delete or /account delete force".bright_blue());
            Command::InvalidSyntax { err_msg }
        }

        ["room", "create", name] |
        ["r", "create", name] |
        ["room", "c", name] |
        ["r", "c", name] => Command::RoomCreate {
            name: name.to_string(),
            whitelist: false
        },

        ["room", "create", name, "whitelist"] |
        ["r", "create", name, "whitelist"] |
        ["room", "c", name, "whitelist"] |
        ["r", "c", name, "whitelist"] |
        ["room", "create", name, "w"] |
        ["r", "create", name, "w"] |
        ["room", "c", name, "w"] |
        ["r", "c", name, "w"] |
        ["room", "create", name, "private"] |
        ["r", "create", name, "private"] |
        ["room", "c", name, "private"] |
        ["r", "c", name, "private"] |
        ["room", "create", name, "p"] |
        ["r", "create", name, "p"] |
        ["room", "c", name, "p"] |
        ["r", "c", name, "p"] => Command::RoomCreate {
            name: name.to_string(),
            whitelist: true
        },

        ["room", "create", ..] |
        ["r", "create", ..] |
        ["room", "c", ..] |
        ["r", "c", ..] => {
            let err_msg = format!("{}", "Usage: /room create <room name> or /room create <room name> whitelist".bright_blue());
            Command::InvalidSyntax { err_msg }
        },

        ["room", "join", name] |
        ["r", "join", name] |
        ["room", "j", name] |
        ["r", "j", name] => Command::RoomJoin {
            name: name.to_string(),
        },

        ["room", "join", ..] |
        ["r", "join", ..] |
        ["room", "j", ..] |
        ["r", "j", ..] => {
            let err_msg = format!("{}", "Usage: /room join <room name>".bright_blue());
            Command::InvalidSyntax { err_msg }
        },

        ["room", "import", filename] |
        ["r", "import", filename] => Command::RoomImport {
            filename: filename.to_string()
        },

        ["room", "import", ..] |
        ["r", "import", ..] => {
            let err_msg = format!("{}", "Usage: /room import <filename>".bright_blue());
            Command::InvalidSyntax { err_msg }
        },

        ["room", "delete", name] |
        ["r", "delete", name] | 
        ["room", "d", name] |
        ["r", "d", name] => Command::RoomDelete{
            name: name.to_string(),
            force: false
        },

        ["room", "delete", "force", name] |
        ["r", "delete", "force", name] |
        ["room", "d", "force", name] |
        ["room", "delete", "f", name] |
        ["r", "d", "force", name] |
        ["r", "delete", "f", name] |
        ["room", "d", "f", name] |
        ["r", "d", "f", name] => Command::RoomDelete {
            name: name.to_string(),
            force: true
        },

        ["room", "delete", ..] |
        ["r", "delete", ..] |
        ["room", "d", ..] |
        ["r", "d", ..] => {
            let err_msg = format!("{}", "Usage: /room delete <room name> or /room delete <room name> force".bright_blue());
            Command::InvalidSyntax { err_msg }
        },

        ["room", "list"] |
        ["r", "list"] |
        ["room", "l"] |
        ["r", "l"] => Command::RoomList,

        ["room", "list", ..] |
        ["r", "list", ..] |
        ["room", "l", ..] |
        ["r", "l", ..] => {
            let err_msg = format!("{}", "Usage: /room list".bright_blue());
            Command::InvalidSyntax { err_msg }
        },

        ["room", ..] |
        ["r", ..] => {
            let err_msg = format!("{}", "Room commands:\n> /room list\n> /room create <room name> or /room create <room name> whitelist\n> /room join <room name>\n> /room import <filename>\n> /room delete <room name> or /room delete <room name> force".bright_blue());
            Command::InvalidSyntax { err_msg }
        },

        _ => Command::Unavailable
    }
}
