# Get the closest Web3 RPC provider in Rust
[![Rust CI](https://github.com/samuelsramko/rust-web3-closest-provider/actions/workflows/rust_ci.yml/badge.svg)](https://github.com/samuelsramko/rust-web3-closest-provider/actions/workflows/rust_ci.yml)
## Introduction

Tired of manually switching between Web3 RPC providers to find the fastest one? This library takes the hassle out of it by automatically load balancing between your chosen providers based on their response times. Simply provide a list of potential providers, and the library will dynamically select the fastest option for your local setup, ensuring optimal performance for your Web3 applications.

## Key Features

* **Automatic Response Time Checks:** The library periodically checks the response times of each provider in your list, keeping your selection up-to-date.
* **Dynamic Selection:** Based on the latest response times, the library seamlessly chooses the fastest provider, ensuring you're always using the best option.
* **Easy Integration:** Integrate this library into your Web3 applications quickly and effortlessly using its straightforward API.
* **Customizable Interval:** Adjust the frequency of response time checks to fit your specific needs and network conditions.
* **Clear Communication:** The library logs information about selected providers and encountered errors, keeping you informed.

## Installation

Using Cargo:

1. Add `web3_closest_provider = "1.0.0"` to your `Cargo.toml` dependencies.
2. or 
```bash 
cargo add web3_closest_provider
```

## Usage

1. **Import the library:**

```rust
use web3_closest_provider::{ClosestWeb3Provider, ClosestWeb3RpcProviderSelector};
```

2. **Create a list of potential providers:**
```rust
let providers = vec![
    "[https://mainnet.infura.io/v3/your_api_key](https://mainnet.infura.io/v3/your_api_key)".to_string(),
    "[https://rpc.ankr.com/eth](https://rpc.ankr.com/eth)".to_string(),
    "[https://api.mycryptoapi.com/v1/eth](https://api.mycryptoapi.com/v1/eth)".to_string(),
];
```

3. **Initialize the load balancer:**
```rust
let balancer = ClosestWeb3RpcProviderSelector::init(providers.clone(), std::time::Duration::from_secs(10));
```

4. **Check if the balancer is ready:**
```rust
if balancer.is_ready() {
    println!("Balancer is ready to use!");
} else {
    balancer.wait_until_ready().await;
    println!("Balancer is ready to use!");
}
```

5. **Get the URL of the fastest provider:**
```rust
let fastest_provider = balancer.get_fastest_provider();
println!("Fastest provider: {}", fastest_provider);

// ... use the fastest provider for your Web3 operations ...
```

6. **Crucially, remember to always destroy the balancer when you're done:**
```rust
balancer.destroy(); // **This step is essential!**
```

## Customization
* You can change the interval_duration to adjust the frequency of response time checks.
* You can implement specific checks or logic for provider selection beyond response time.

## Example Usage
```rust
use web3_closest_provider::{ClosestWeb3Provider, ClosestWeb3RpcProviderSelector};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let providers = vec![
        "[https://mainnet.infura.io/v3/your_api_key](https://mainnet.infura.io/v3/your_api_key)".to_string(),
        "[https://rpc.ankr.com/eth](https://rpc.ankr.com/eth)".to_string(),
        "[https://api.mycryptoapi.com/v1/eth](https://api.mycryptoapi.com/v1/eth)".to_string(),
    ];

    let balancer = ClosestWeb3RpcProviderSelector::init(providers.clone(), Duration::from_secs(10));

    balancer.wait_until_ready().await;
    let fastest_provider = balancer.get_fastest_provider();
    println!("Using fastest provider: {}", fastest_provider);

    // ... use the fastest provider for your Web3 operations ...

    // **Remember to destroy the balancer!**
    balancer.destroy();
}
```

## Contribution
We welcome contributions! Please refer to the [CONTRIBUTING.md](./CONTRIBUTING.md) file for guidelines.

## License
This library is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.