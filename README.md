
# Stylus Chainlink Price Feeds

Chainlink Price Feeds for Arbitrum adn Robinhood.

## Getting a price feed for BtcUsd (using stylus-sdk)

```rust
use stylus_chainlink_price_feeds::{robinhood::Robinhood, get_latest_round_data_opt};

// This returns Some(x) if the call worked:
get_latest_round_data_bool(vm, vm, Robinhood::BtcUsd).unwrap()
```
