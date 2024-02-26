// Standard library modules
use std::{
    collections::HashMap,
    error::Error,
    fmt,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

// External libraries
use serde::Deserialize;
use serde_json::Value;
use tokio::{sync::watch, time::sleep};

/// Represents a JSON-RPC response with an optional error field.
#[derive(Debug, Deserialize)]
struct JsonRpcResponse {
    /// Optional error message or object.
    error: Option<Value>,
}

/// A custom error type for representing errors within the library.
#[derive(Debug)]
struct LibError {
    /// The error message.
    message: String,
}

impl fmt::Display for LibError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for LibError {}

/// Defines methods for interacting with a Web3 provider balancer.
/// This trait enables you to:
/// * Initialize a balancer with a list of Web3 provider URLs.
/// * Check if the balancer is ready to provide the fastest provider.
/// * Stop the balancer and clear its data.
/// * Get the URL of the provider with the fastest response time.
/// * Wait until the balancer is ready to provide the fastest provider.
///
/// This trait is useful for applications that need to dynamically select the fastest Web3 provider based on response time, ensuring optimal performance for your Web3 operations.
pub trait ClosestWeb3Provider {
    /// Initializes the provider balancer with a list of URLs.
    ///
    /// # Example
    ///
    /// ```
    /// use web3_closest_provider::{ClosestWeb3Provider, ClosestWeb3RpcProviderSelector};
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let providers = vec![
    ///         "https://mainnet.infura.io/v3/your_api_key".to_string(),
    ///         "https://rpc.ankr.com/eth".to_string(),
    ///         "https://api.mycryptoapi.com/v1/eth".to_string(),
    ///     ];
    ///
    ///     let balancer = ClosestWeb3RpcProviderSelector::init(providers.clone(), Duration::from_secs(10));
    /// }
    /// ```
    ///
    /// # Arguments
    ///
    /// * `urls` - A vector of URLs for the Web3 providers.
    /// * `checking_interval` - The interval at which the balancer checks the response times of the providers.
    fn init(urls: Vec<String>, checking_interval: Duration) -> Self;

    /// Checks if the balancer is ready to provide the fastest provider.
    ///
    /// # Example
    ///
    /// ```
    /// use web3_closest_provider::{ClosestWeb3Provider, ClosestWeb3RpcProviderSelector};
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let providers = vec![
    ///         "https://mainnet.infura.io/v3/your_api_key".to_string(),
    ///         "https://rpc.ankr.com/eth".to_string(),
    ///         "https://api.mycryptoapi.com/v1/eth".to_string(),
    ///     ];
    ///
    ///     let balancer = ClosestWeb3RpcProviderSelector::init(providers.clone(), Duration::from_secs(10));
    ///
    ///     if balancer.is_ready() {
    ///         println!("Balancer is ready to use!");
    ///     } else {
    ///         balancer.wait_until_ready().await;    
    ///         println!("Balancer is ready to use!");
    ///     }
    /// }
    /// ```
    ///
    /// # Returns
    ///
    /// * `true` if the balancer is ready; `false` otherwise.
    fn is_ready(&self) -> bool;

    /// Stops the balancer and clears its data.
    ///
    /// # Example
    ///
    /// ```
    /// use web3_closest_provider::{ClosestWeb3Provider, ClosestWeb3RpcProviderSelector};
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let providers = vec![
    ///         "https://mainnet.infura.io/v3/your_api_key".to_string(),
    ///         "https://rpc.ankr.com/eth".to_string(),
    ///         "https://api.mycryptoapi.com/v1/eth".to_string(),
    ///     ];
    ///
    ///     let balancer = ClosestWeb3RpcProviderSelector::init(providers.clone(), Duration::from_secs(10));
    ///
    ///     balancer.destroy(); // **This step is essential!**
    /// }
    /// ```
    fn destroy(&self);

    /// Returns the URL of the provider with the fastest response time.
    ///
    /// # Example
    ///
    /// ```
    /// use web3_closest_provider::{ClosestWeb3Provider, ClosestWeb3RpcProviderSelector};
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let providers = vec![
    ///         "https://mainnet.infura.io/v3/your_api_key".to_string(),
    ///         "https://rpc.ankr.com/eth".to_string(),
    ///         "https://api.mycryptoapi.com/v1/eth".to_string(),
    ///     ];
    ///
    ///     let balancer = ClosestWeb3RpcProviderSelector::init(providers.clone(), Duration::from_secs(10));
    ///
    ///     balancer.wait_until_ready().await;   
    ///     let fastest_provider = balancer.get_fastest_provider();
    ///     println!("Fastest provider: {}", fastest_provider);
    /// }
    ///
    /// // ... use the fastest provider for your Web3 operations ...
    /// ```
    ///
    /// # Returns
    ///
    /// The URL of the provider with the fastest response time.
    ///
    /// # Panics
    ///
    /// This function will panic if the hashmap containing response times is empty.
    fn get_fastest_provider(&self) -> String;

    /// Waits until the balancer is ready to provide the fastest provider.
    ///
    /// # Example
    ///
    /// ```
    /// use web3_closest_provider::{ClosestWeb3Provider, ClosestWeb3RpcProviderSelector};
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let providers = vec![
    ///         "https://mainnet.infura.io/v3/your_api_key".to_string(),
    ///         "https://rpc.ankr.com/eth".to_string(),
    ///         "https://api.mycryptoapi.com/v1/eth".to_string(),
    ///     ];
    ///
    ///     let balancer = ClosestWeb3RpcProviderSelector::init(providers.clone(), Duration::from_secs(10));
    ///
    ///     balancer.wait_until_ready().await;
    ///     println!("Balancer is ready to use!");
    /// }
    /// ```
    fn wait_until_ready(&self) -> impl std::future::Future<Output = ()> + Send;
}

/// A concrete implementation of the `ClosestWeb3Provider` trait that balances Web3 providers based on their response times.
/// This struct:
/// * Internally tracks response times for each provided URL.
/// * Periodically checks response times to update its internal map.
/// * Provides methods to access the fastest provider and its URL.
/// * Allows waiting until the fastest provider is available.
///
/// This implementation offers a convenient way to manage and utilize multiple Web3 providers while ensuring optimal performance.
pub struct ClosestWeb3RpcProviderSelector {
    /// Sender for sending messages to the response time check task.
    interval_handle: watch::Sender<()>,

    /// Shared map storing the response time for each provider.
    current_response_time_per_url: Arc<Mutex<HashMap<String, u128>>>,
}

impl ClosestWeb3Provider for ClosestWeb3RpcProviderSelector {
    fn init(urls: Vec<String>, checking_interval: Duration) -> Self {
        // Create a channel for sending messages to the response time check task.
        let (tx, rx) = watch::channel(());

        // Create a shared map to store response times.
        let current_response_time_per_url = Arc::new(Mutex::new(HashMap::new()));

        // Spawn a task to periodically check response times.
        tokio::spawn(Self::process_response_time_check(
            urls.clone(),
            rx,
            current_response_time_per_url.clone(),
            checking_interval,
        ));

        // Return the ClosestWeb3RpcProviderSelector instance.
        ClosestWeb3RpcProviderSelector {
            interval_handle: tx,
            current_response_time_per_url,
        }
    }

    fn is_ready(&self) -> bool {
        // Check if the response time map has any entries.
        self.current_response_time_per_url.lock().unwrap().len() > 0
    }

    fn destroy(&self) {
        // Send a message to stop the response time check task.
        self.interval_handle
            .send(())
            .expect("Failed to send DESTROY message to interval_handle");

        // Clear the response time map.
        self.current_response_time_per_url.lock().unwrap().clear();
    }

    fn get_fastest_provider(&self) -> String {
        // Lock the response time map and find the provider with the lowest response time.
        let binding = self.current_response_time_per_url.lock().unwrap();
        let (key, _) = binding.iter().min_by_key(|(_, &v)| v).unwrap();

        // Clone and return the URL of the fastest provider.
        key.clone()
    }

    async fn wait_until_ready(&self) {
        loop {
            if self.is_ready() {
                break;
            }
            sleep(Duration::from_millis(10)).await;
        }
    }
}

impl ClosestWeb3RpcProviderSelector {
    /// Asynchronously checks the response times of the providers and updates the response time map.
    async fn process_response_time_check(
        urls: Vec<String>,
        receiver: watch::Receiver<()>,
        response_times: Arc<Mutex<HashMap<String, u128>>>,
        checking_interval: Duration,
    ) {
        loop {
            // Clone the receiver to avoid borrowing issues within the select macro.
            let mut receiver_clone = receiver.clone();

            // Select between different branches based on received messages or timeouts.

            tokio::select! {
                // Handle a message from the receiver indicating destruction.
                _ = receiver_clone.changed() => {
                    break;
                }

                // Perform a request to one of the URLs concurrently.
                _ = async {
                    for url in &urls {
                        let response = Self::perform_web3_client_version_request(&url).await;
                        let response_time = response.unwrap_or(u128::MAX);

                        // Acquire a lock on the response time map and update the value.
                        let mut response_times_map = response_times.lock().unwrap();
                        response_times_map.insert(url.clone(), response_time);
                        drop(response_times_map);
                    }
                } => {}

                // Wait for the interval duration to pass.
                _ = sleep(checking_interval) => {}
            }
        }
    }

    /// Sends a JSON-RPC request to a given URL and returns the response time or an error.
    async fn perform_web3_client_version_request(url: &str) -> Result<u128, LibError> {
        let client = reqwest::Client::new();

        // Prepare the JSON-RPC request body.
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "web3_clientVersion",
            "params": [],
            "id": 1
        });

        // Record the start time of the request.
        let start_time = Instant::now();

        // Send the request and handle potential errors.
        let response = client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| LibError {
                message: format!("Failed to send request: {:?}", e),
            })?;

        // Record the end time of the request.
        let end_time = Instant::now();

        // Check if the response contains an error field.
        let json_response: JsonRpcResponse = response.json().await.map_err(|e| LibError {
            message: format!("Failed to parse response: {:?}", e),
        })?;

        if let Some(error) = json_response.error {
            return Err(LibError {
                message: format!("Received error response: {:?}", error),
            });
        }

        // Calculate and return the response time.
        Ok(end_time.duration_since(start_time).as_micros())
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClosestWeb3Provider, ClosestWeb3RpcProviderSelector};
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_init() {
        let urls = vec![
            "https://eth.llamarpc.com".to_string(),
            "https://eth.llamarpc.com".to_string(),
        ];
        let provider = ClosestWeb3RpcProviderSelector::init(urls.clone(), Duration::from_secs(10));
        assert_eq!(provider.is_ready(), false);
        provider.wait_until_ready().await;
        assert_eq!(provider.is_ready(), true);
        assert_eq!(
            provider.get_fastest_provider(),
            "https://eth.llamarpc.com".to_string()
        );
    }

    #[tokio::test]
    async fn test_destroy() {
        let urls = vec![
            "https://eth.llamarpc.com".to_string(),
            "https://eth.llamarpc.com".to_string(),
        ];
        let provider = ClosestWeb3RpcProviderSelector::init(urls.clone(), Duration::from_secs(10));
        // Check that the interval handle was created successfully
        assert_eq!(provider.is_ready(), false);
        provider.wait_until_ready().await;
        assert_eq!(provider.is_ready(), true);
        assert_eq!(
            provider.get_fastest_provider(),
            "https://eth.llamarpc.com".to_string()
        );

        // Destroy the provider
        provider.destroy();
        sleep(Duration::from_millis(1000)).await;
        assert_eq!(provider.is_ready(), false);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_destroy_and_panic_after_reading_provider_from_destroyed_instance() {
        let urls = vec![
            "https://eth.llamarpc.com".to_string(),
            "https://eth.llamarpc.com".to_string(),
        ];
        let provider = ClosestWeb3RpcProviderSelector::init(urls.clone(), Duration::from_secs(10));
        // Check that the interval handle was created successfully
        assert_eq!(provider.is_ready(), false);
        provider.wait_until_ready().await;
        assert_eq!(provider.is_ready(), true);
        assert_eq!(
            provider.get_fastest_provider(),
            "https://eth.llamarpc.com".to_string()
        );

        // Destroy the provider
        provider.destroy();
        sleep(Duration::from_millis(1000)).await;
        assert_eq!(provider.is_ready(), false);
        provider.get_fastest_provider();
    }

    #[tokio::test]
    async fn test_provider_with_multiple_requests() {
        let urls: Vec<String> = vec![
            "https://eth.llamarpc.com",
            "https://rpc.lokibuilder.xyz/wallet",
            "wss://ethereum.publicnode.com",
            "wss://mainnet.gateway.tenderly.co",
            "https://gateway.tenderly.co/public/mainnet",
            "https://core.gashawk.io/rpc",
            "https://mainnet.gateway.tenderly.co",
            "https://virginia.rpc.blxrbdn.com",
            "https://uk.rpc.blxrbdn.com",
            "https://singapore.rpc.blxrbdn.com",
        ]
        .iter()
        .map(|&s| s.to_string())
        .collect();
        let provider = ClosestWeb3RpcProviderSelector::init(urls.clone(), Duration::from_secs(2));
        provider.wait_until_ready().await;
        assert_eq!(provider.is_ready(), true);
        for _ in 0..3 {
            let fastest_provider = provider.get_fastest_provider();
            println!("Fastest provider: {}", fastest_provider);
            sleep(Duration::from_millis(2400)).await;
        }
        // Destroy the provider
        provider.destroy();
        sleep(Duration::from_millis(1000)).await;
        assert_eq!(provider.is_ready(), false);
    }
}
