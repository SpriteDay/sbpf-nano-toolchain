use std::sync::LazyLock;

pub use jobserver_crate::Client;
use jobserver_crate::{FromEnv, FromEnvErrorKind};

static GLOBAL_CLIENT: LazyLock<Result<Client, String>> = LazyLock::new(|| {
    let FromEnv { client, var } = unsafe { Client::from_env_ext(true) };

    let error = match client {
        Ok(client) => return Ok(client),
        Err(e) => e,
    };

    if matches!(
        error.kind(),
        FromEnvErrorKind::NoEnvVar
            | FromEnvErrorKind::NoJobserver
            | FromEnvErrorKind::NegativeFd
            | FromEnvErrorKind::Unsupported
    ) {
        return Ok(default_client());
    }

    let (name, value) = var.unwrap();
    Err(format!(
        "Failed to connect to jobserver from environment variable `{name}={:?}`: {error}",
        value
    ))
});

fn default_client() -> Client {
    let client = Client::new(32).expect("Failed to create jobserver");

    client.acquire_raw().ok();

    client
}

pub fn initialize_checked(report_warning: impl FnOnce(&'static str)) {
    // let client_checked = match
}
