#[cfg(feature = "enable-log")]
use easy_log::{EasyLog, EasyLogConfig};

#[cfg(feature = "enable-log")]
mod demo_log_config;

#[cfg(feature = "enable-log")]
use demo_log_config::DemoDebugLogConfig;

///
struct TcpServer {}

///
impl TcpServer {
    ///
    pub fn on_receive_data() {
        // Get the singleton log config
        #[cfg(feature = "enable-log")]
        let log_config = DemoDebugLogConfig::get_easy_log_config_from_env();

        // normal logic code...

        // Print the optional debug log if `TCP_ENABLE_PROTOCOL_DEBUG_LOG` available
        #[cfg(feature = "enable-log")]
        if log_config.tcp_enable_protocol_debug_log {
            EasyLog::debug_log(
                "TcpServer",
                "on_receive_data",
                "debug protocol: 7E0011FF",
                None,
            );
        }
    }
}

///
struct NetworkRequestHandler {}

///
impl NetworkRequestHandler {
    ///
    pub fn network_request_handler() {
        // Get the singleton log config
        #[cfg(feature = "enable-log")]
        let log_config = DemoDebugLogConfig::get_easy_log_config_from_env();

        // Print necessary the info log
        #[cfg(feature = "enable-log")]
        {
            EasyLog::info_log(
                "NetworkRequestHandler",
                "network_request_handler",
                "Start handle request......",
                None,
            );
        }

        // normal logic code...

        // Print the optional debug log if `ENABLE_REQUEST_LOG` available
        #[cfg(feature = "enable-log")]
        if log_config.network_handler_enable_request_debug_log {
            EasyLog::debug_log(
                "NetworkRequestHandler",
                "network_request_handler",
                "request: {}",
                None,
            );
        }

        // normal logic code...

        // Print the optional debug log if `ENABLE_BODY_LOG` available
        #[cfg(feature = "enable-log")]
        if log_config.network_handler_enable_body_debug_log {
            EasyLog::debug_log(
                "NetworkRequestHandler",
                "network_request_handler",
                "body: {}",
                None,
            );
        }

        // normal logic code...

        // Print the optional debug log if `ENABLE_NETWORK_LOG` available
        #[cfg(feature = "enable-log")]
        if log_config.network_handler_enable_network_debug_log {
            EasyLog::debug_log(
                "NetworkRequestHandler",
                "network_request_handler",
                "network: {}",
                None,
            );
        }

        // Print necessary the info log
        #[cfg(feature = "enable-log")]
        {
            EasyLog::info_log(
                "NetworkRequestHandler",
                "network_request_handler",
                "End handle request......",
                None,
            );
        }
    }
}

///
fn main() {
    #[cfg(feature = "enable-log")]
    {
        println!("Enable easy log functionality >>>>>>>>>.");
        // println!("EasyLog singleton instance: {:#?}", EasyLog::get_logger());

        let log_config = DemoDebugLogConfig::get_easy_log_config_from_env();
        println!("log_config {:#?}", log_config);
    }

    NetworkRequestHandler::network_request_handler();
    TcpServer::on_receive_data();

    println!("Demo done.");
}
