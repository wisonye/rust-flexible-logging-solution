#[cfg(feature = "enable-log")]
use std::env;

#[cfg(feature = "enable-log")]
use easy_log::EasyLogConfig;

#[cfg(feature = "enable-log")]
#[derive(Debug)]
pub struct DemoDebugLogConfig {
    pub network_handler_enable_request_debug_log: bool,
    pub network_handler_enable_body_debug_log: bool,
    pub network_handler_enable_network_debug_log: bool,
    pub tcp_enable_protocol_debug_log: bool
}

#[cfg(feature = "enable-log")]
impl DemoDebugLogConfig {
    ///
    pub fn get_config() -> &'static mut Self {
        unsafe {
            if SINGLETON_INSTANCE.is_none() {
                SINGLETON_INSTANCE = Some(DemoDebugLogConfig {
                    network_handler_enable_request_debug_log: match env::var("NETWORK_HANDLER_ENABLE_REQUEST_DEBUG_LOG") {
                        Ok(str_value) => str_value.to_uppercase() == "TRUE",
                        Err(_) => false,
                    },
                    network_handler_enable_body_debug_log: match env::var("NETWORK_HANDLER_ENABLE_BODY_DEBUG_LOG") {
                        Ok(str_value) => str_value.to_uppercase() == "TRUE",
                        Err(_) => false,
                    },
                    network_handler_enable_network_debug_log: match env::var("NETWORK_HANDLER_ENABLE_NETWORK_DEBUG_LOG") {
                        Ok(str_value) => str_value.to_uppercase() == "TRUE",
                        Err(_) => false,
                    },
                    tcp_enable_protocol_debug_log: match env::var("TCP_ENABLE_PROTOCOL_DEBUG_LOG") {
                        Ok(str_value) => str_value.to_uppercase() == "TRUE",
                        Err(_) => false,
                    },
                });
                println!("Create singletoninstance >>>>>>>>>>>>>>>>>");
            }
            SINGLETON_INSTANCE.as_mut().unwrap()
        }
    }
}

#[cfg(feature = "enable-log")]
impl EasyLogConfig<DemoDebugLogConfig> for DemoDebugLogConfig {
    ///
    fn get_easy_log_config_from_env() -> &'static DemoDebugLogConfig {
        Self::get_config()
    }
}

#[cfg(feature = "enable-log")]
static mut SINGLETON_INSTANCE: Option<DemoDebugLogConfig> = None;
