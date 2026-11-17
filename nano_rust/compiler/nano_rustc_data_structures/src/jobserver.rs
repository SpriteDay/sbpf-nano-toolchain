use std::sync::{LazyLock, OnceLock};

pub use jobserver_crate::Client;
use jobserver_crate::{FromEnv, FromEnvErrorKind};

// We can only call `from_env_ext` once per process

// We stick this in a global because there could be multiple nano_rustc instances
// in this process, and the jobserver is per-process
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

    // Environment specifies jobserver but it looks incorrect
    let (name, value) = var.unwrap();
    Err(format!(
        "Failed to connect to jobserver from environment variable `{name}={:?}`: {error}",
        value
    ))
});

// Create a new jobserver if there's no inherited one
fn default_client() -> Client {
    // Pick a "reasonable maximum" capping out at 32
    // so we don't take everything down by hogging the process run queue.
    // The fixed number is used to have determenistic compilation across machines
    let client = Client::new(32).expect("Failed to create jobserver");

    // Accquire a token for the main thread which we can release later
    client.acquire_raw().ok();

    client
}

static GLOBAL_CLIENT_CHECKED: OnceLock<Client> = OnceLock::new();

pub fn initialize_checked(report_warning: impl FnOnce(&'static str)) {
    let client_checked = match &*GLOBAL_CLIENT {
        Ok(client) => client.clone(),
        Err(e) => {
            report_warning(e);
            default_client()
        }
    };
    GLOBAL_CLIENT_CHECKED.set(client_checked).ok();
}
