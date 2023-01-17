use crate::responses::{EmailSendResponse, UserError};
use log::warn;
use std::{env, net::SocketAddr};

const DEFAULT_PORT: u16 = 8080;

pub struct EnvVars {}

impl EnvVars {
    pub fn get_sendgrid_api_key() -> Result<String, UserError> {
        Self::get_env_variable("SENDGRID_API_KEY", "Required env variable not set")
    }

    pub fn get_send_from_email() -> Result<String, UserError> {
        Self::get_env_variable("SEND_FROM_EMAIL", "Required env variable not set")
    }

    pub fn get_send_to_email() -> Result<String, UserError> {
        Self::get_env_variable("SEND_TO_EMAIL", "Required env variable not set")
    }

    pub fn get_env_variable(env_variable: &str, error_message: &str) -> Result<String, UserError> {
        let env_value = env::var(env_variable).map_err(|_| UserError::InternalServerError {
            body: EmailSendResponse::error(error_message, Some("Internal Server Error")),
        })?;

        Ok(env_value)
    }
}

pub fn get_socket_addr() -> SocketAddr {
    SocketAddr::from((
        [0, 0, 0, 0],
        env::var("PORT")
            .unwrap_or_else(|_| {
                warn!(
                    "PORT not found .env file, using default port: {}",
                    DEFAULT_PORT
                );
                DEFAULT_PORT.to_string()
            })
            .parse::<u16>()
            .unwrap_or_else(|_| {
                warn!(
                    "PORT is not a valid port number, using default port: {}",
                    DEFAULT_PORT
                );
                DEFAULT_PORT
            }),
    ))
}
