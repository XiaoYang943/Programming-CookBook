use std::time::Duration;

#[derive(Debug)]
struct RawConfig {
    endpoint: String,
    timeout_seconds: Option<u64>,
    proxy_url: Option<String>,
}

#[derive(Debug)]
struct RuntimeConfig {
    endpoint: String,
    timeout: Duration,
    proxy_url: Option<String>,
}

impl RawConfig {
    fn into_runtime(self) -> RuntimeConfig {
        RuntimeConfig {
            endpoint: self.endpoint,
            timeout: Duration::from_secs(self.timeout_seconds.unwrap_or(30)),
            proxy_url: self.proxy_url,
        }
    }
}

fn main() {
    let config = RawConfig {
        endpoint: "https://api.example.com".to_string(),
        timeout_seconds: None,
        proxy_url: Some("http://proxy.internal:8080".to_string()),
    }
    .into_runtime();

    println!("endpoint: {}", config.endpoint);
    println!("timeout: {:?}", config.timeout);

    match config.proxy_url {
        Some(proxy) => println!("proxy: {proxy}"),
        None => println!("proxy disabled"),
    }
}
