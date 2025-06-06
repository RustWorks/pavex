use jsonwebtoken::{DecodingKey, EncodingKey};
use pavex::server::IncomingStream;
use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

/// Configuration for the HTTP server used to expose our API
/// to users.
#[derive(serde::Deserialize, Debug, Clone)]
#[pavex::config(key = "server", include_if_unused)]
pub struct ServerConfig {
    /// The port that the server must listen on.
    ///
    /// Set the `PX_SERVER__PORT` environment variable to override its value.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    /// The network interface that the server must be bound to.
    ///
    /// E.g. `0.0.0.0` for listening to incoming requests from
    /// all sources.
    ///
    /// Set the `PX_SERVER__IP` environment variable to override its value.
    pub ip: std::net::IpAddr,
    /// The timeout for graceful shutdown of the server.
    ///
    /// E.g. `1 minute` for a 1 minute timeout.
    ///
    /// Set the `PX_SERVER__GRACEFUL_SHUTDOWN_TIMEOUT` environment variable to override its value.
    #[serde(deserialize_with = "deserialize_shutdown")]
    pub graceful_shutdown_timeout: std::time::Duration,
}

fn deserialize_shutdown<'de, D>(deserializer: D) -> Result<std::time::Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize as _;

    let duration = pavex::time::SignedDuration::deserialize(deserializer)?;
    if duration.is_negative() {
        Err(serde::de::Error::custom(
            "graceful shutdown timeout must be positive",
        ))
    } else {
        duration.try_into().map_err(serde::de::Error::custom)
    }
}

impl ServerConfig {
    /// Bind a TCP listener according to the specified parameters.
    pub async fn listener(&self) -> Result<IncomingStream, std::io::Error> {
        let addr = std::net::SocketAddr::new(self.ip, self.port);
        IncomingStream::bind(addr).await
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
#[pavex::config(key = "database")]
pub struct DatabaseConfig {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

#[pavex::methods]
impl DatabaseConfig {
    /// Return the database connection options.
    pub fn connection_options(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
            .database(&self.database_name)
    }

    /// Return a database connection pool.
    #[singleton(clone_if_necessary)]
    pub async fn get_pool(&self) -> Result<sqlx::PgPool, sqlx::Error> {
        let pool = sqlx::PgPool::connect_with(self.connection_options()).await?;
        Ok(pool)
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
#[pavex::config(key = "auth")]
/// Configuration for the authentication system.
pub struct AuthConfig {
    /// The private key used to sign JWTs.
    pub eddsa_private_key_pem: Secret<String>,
    /// The public key used to verify the signature of JWTs.
    pub eddsa_public_key_pem: String,
}

#[pavex::methods]
impl AuthConfig {
    /// Return the private key to be used for JWT signing.
    #[singleton]
    pub fn encoding_key(&self) -> Result<EncodingKey, jsonwebtoken::errors::Error> {
        EncodingKey::from_ed_pem(self.eddsa_private_key_pem.expose_secret().as_bytes())
    }

    /// Return the public key to be used for verifying the signature of JWTs.
    pub fn decoding_key(&self) -> Result<DecodingKey, jsonwebtoken::errors::Error> {
        DecodingKey::from_ed_pem(self.eddsa_public_key_pem.as_bytes())
    }
}
