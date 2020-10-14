use colored::*;
use std::env;

///
pub trait EasyLogConfig<T> {
    fn get_easy_log_config_from_env() -> &'static T;
}

///
#[derive(Debug, PartialEq, Eq)]
pub enum LogLevel {
    DEBUG = 1,
    INFO = 2,
    WARNING = 3,
    ERROR = 4,
}

///
#[derive(Debug)]
pub struct EasyLog {}

/// Singleton mutable instance
// static mut SINGLETON_INSTANCE: Option<EasyLog> = None;

///
fn print_log(
    log_level: LogLevel,
    log_name: &str,
    method_name: &str,
    msg: &str,
    extra_msg: Option<&str>,
) {
    let log_level_from_env_str = match env::var("LOG_LEVEL") {
        Ok(level) => level,
        Err(_) => String::from("ERROR"),
    };

    let log_level_from_env = match log_level_from_env_str.to_uppercase().as_str() {
        "DEBUG" => LogLevel::DEBUG,
        "INFO" => LogLevel::INFO,
        "WARNING" => LogLevel::WARNING,
        "ERROR" => LogLevel::ERROR,
        _ => LogLevel::ERROR,
    };

    let log_level_from_env_int = log_level_from_env as u8;
    let log_level_int = log_level as u8;
    let enable_log = log_level_from_env_int <= log_level_int;

    // println!("log_level_from_env_int: {:?}", log_level_from_env_int);
    // println!("log_level_int: {:?}", log_level_int);
    // println!("enable_log: {}", enable_log);

    if !enable_log {
        return;
    }

    if let Some(temp_msg) = extra_msg {
        let color_content = match log_level_int {
            1 => (
                format!(
                    "{} {} - {} {} ",
                    "[".blue(),
                    log_name.blue(),
                    method_name.blue().bold(),
                    "]".blue()
                ),
                msg.white(),
                temp_msg.white(),
            ),
            2 => (
                format!(
                    "{} {} - {} {} ",
                    "[".green(),
                    log_name.green(),
                    method_name.green().bold(),
                    "]".green()
                ),
                msg.green(),
                temp_msg.green(),
            ),
            3 => (
                format!(
                    "{} {} - {} {} ",
                    "[".yellow(),
                    log_name.yellow(),
                    method_name.yellow().bold(),
                    "]".yellow()
                ),
                msg.yellow(),
                temp_msg.yellow(),
            ),
            4 => (
                format!(
                    "{} {} - {} {} ",
                    "[".red(),
                    log_name.red(),
                    method_name.red().bold(),
                    "]".red()
                ),
                msg.red(),
                temp_msg.red(),
            ),
            _ => (
                format!(
                    "{} {} - {} {} ",
                    "[".cyan(),
                    log_name.cyan(),
                    method_name.cyan().bold(),
                    "]".cyan()
                ),
                msg.white(),
                temp_msg.white(),
            ),
        };

        println!("{} {}{}", color_content.0, color_content.1, color_content.2);
    } else {
        let color_content = match log_level_int {
            1 => (
                format!(
                    "{} {} - {} {} ",
                    "[".blue(),
                    log_name.blue(),
                    method_name.blue().bold(),
                    "]".blue()
                ),
                msg.white(),
            ),
            2 => (
                format!(
                    "{} {} - {} {} ",
                    "[".green(),
                    log_name.green(),
                    method_name.green().bold(),
                    "]".green()
                ),
                msg.green(),
            ),
            3 => (
                format!(
                    "{} {} - {} {} ",
                    "[".yellow(),
                    log_name.yellow(),
                    method_name.yellow().bold(),
                    "]".yellow()
                ),
                msg.yellow(),
            ),
            4 => (
                format!(
                    "{} {} - {} {} ",
                    "[".red(),
                    log_name.red(),
                    method_name.red().bold(),
                    "]".red()
                ),
                msg.red(),
            ),
            _ => (
                format!(
                    "{} {} - {} {} ",
                    "[".cyan(),
                    log_name.cyan(),
                    method_name.cyan().bold(),
                    "]".cyan()
                ),
                msg.white(),
            ),
        };
        println!("{} {}", color_content.0, color_content.1);
    }
}

///
impl EasyLog {
    // ///
    // pub fn get_logger() -> &'static mut Self {
    // unsafe {
    // if SINGLETON_INSTANCE.is_none() {
    // SINGLETON_INSTANCE = Some(EasyLog {});
    // }
    // SINGLETON_INSTANCE.as_mut().unwrap()
    // }
    // }

    ///
    pub fn get_log_level() -> LogLevel {
        let level_str = match env::var("LOG_LEVEL") {
            Ok(level) => level,
            Err(_) => String::from("INFO"),
        };

        match level_str.to_uppercase().as_str() {
            "DEBUG" => LogLevel::DEBUG,
            "INFO" => LogLevel::INFO,
            "WARNING" => LogLevel::WARNING,
            "ERROR" => LogLevel::ERROR,
            _ => LogLevel::INFO,
        }
    }

    /// Debug log
    pub fn debug_log(log_name: &str, method_name: &str, msg: &str, extra_msg: Option<&str>) {
        print_log(LogLevel::DEBUG, log_name, method_name, msg, extra_msg);
    }

    /// Info log
    pub fn info_log(log_name: &str, method_name: &str, msg: &str, extra_msg: Option<&str>) {
        print_log(LogLevel::INFO, log_name, method_name, msg, extra_msg);
    }

    /// Warning log
    pub fn warning_log(log_name: &str, method_name: &str, msg: &str, extra_msg: Option<&str>) {
        print_log(LogLevel::WARNING, log_name, method_name, msg, extra_msg);
    }

    /// Error log
    pub fn error_log(log_name: &str, method_name: &str, msg: &str, extra_msg: Option<&str>) {
        print_log(LogLevel::ERROR, log_name, method_name, msg, extra_msg);
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_color_theme() {
        println!("{}", "this is blue".blue());
        println!("{}", "this is blue bold".blue().bold());
        println!("{}", "this is blue bright".bright_blue());
        println!("{}", "this is blue bold bright".bright_blue().bold());

        println!("{}", "this is red".red());
        println!("{}", "this is red bold".red().bold());
        println!("{}", "this is red bright".bright_red());
        println!("{}", "this is red bold bright".bright_red().bold());
        println!("{}", "this is yellow".yellow());
        println!("{}", "this is yellow bold".yellow().bold());
        println!("{}", "this is yellow bright".bright_yellow());
        println!("{}", "this is yellow bold bright".bright_yellow().bold());

        println!("{}", "this is green".green());
        println!("{}", "this is green bold".green().bold());
        println!("{}", "this is green bright".bright_green());
        println!("{}", "this is green bold bright".bright_green().bold());

        println!("{}", "this is cyan".cyan());
        println!("{}", "this is cyan bold".cyan().bold());
        println!("{}", "this is cyan bright".bright_cyan());
        println!("{}", "this is cyan bold bright".bright_cyan().bold());

        println!("{}", "this is magenta".magenta());
        println!("{}", "this is magenta bold".magenta().bold());
        println!("{}", "this is magenta bright".bright_magenta());
        println!("{}", "this is magenta bold bright".bright_magenta().bold());
    }

    #[test]
    fn should_not_print_anything() {
        // We set the `LOG_LEVEL` to `INFO`
        env::set_var("LOG_LEVEL", "INFO");

        // We call with `DEBUG` level here, which is not possible to print out, as `DEBUG` < `INFO`
        print_log(
            LogLevel::DEBUG,
            "test_log",
            "test_method",
            "test_message",
            None,
        );
    }

    #[test]
    fn should_print_the_log() {
        env::set_var("LOG_LEVEL", "DEBUG");
        print_log(
            LogLevel::INFO,
            "Tcp Server",
            "start",
            "server start and listening on 0.0.0.0:2019",
            None,
        );
        print_log(
            LogLevel::DEBUG,
            "Tcp Server",
            "on_get_data",
            "7EAABBCCDDFF",
            None,
        );
        print_log(
            LogLevel::WARNING,
            "Tcp Server",
            "on_new_connection",
            "New connection, remote address: 192.168.1.10:8888",
            None,
        );
        print_log(
            LogLevel::ERROR,
            "Tcp Server",
            "on_error",
            "Can read the data from a destroyed socket.",
            None,
        );
    }

    #[test]
    fn should_print_the_log_with_extra_msg() {
        env::set_var("LOG_LEVEL", "DEBUG");
        print_log(
            LogLevel::INFO,
            "Tcp Server",
            "start",
            "server start and listening on: ",
            Some("0.0.0.0:2019".bold().to_string().as_str()),
        );
        print_log(
            LogLevel::DEBUG,
            "Tcp Server",
            "on_get_data",
            "hex: ",
            Some("7EAABBCCDDFF".bold().to_string().as_str()),
        );
        print_log(
            LogLevel::WARNING,
            "Tcp Server",
            "on_new_connection",
            "New connection, remote address: ",
            Some("192.168.1.10:8888".bold().to_string().as_str()),
        );
        print_log(
            LogLevel::ERROR,
            "Tcp Server",
            "on_error",
            "error happened: ",
            Some(
                "Can read the data from a destroyed socket."
                    .bold()
                    .to_string()
                    .as_str(),
            ),
        );
    }

    #[test]
    fn call_specified_logger() {
        env::set_var("LOG_LEVEL", "DEBUG");
        EasyLog::info_log(
            "Tcp Server",
            "start",
            "server start and listening on: ",
            Some("0.0.0.0:2019".bold().to_string().as_str()),
        );
        EasyLog::debug_log(
            "Tcp Server",
            "on_get_data",
            "hex: ",
            Some("7EAABBCCDDFF".bold().to_string().as_str()),
        );
        EasyLog::warning_log(
            "Tcp Server",
            "on_new_connection",
            "New connection, remote address: ",
            Some("192.168.1.10:8888".bold().to_string().as_str()),
        );
        EasyLog::error_log(
            "Tcp Server",
            "on_error",
            "error happened: ",
            Some(
                "Can read the data from a destroyed socket."
                    .bold()
                    .to_string()
                    .as_str(),
            ),
        );
    }
}
