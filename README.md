
# Stylus Chainlink Price Feeds

Chainlink Price Feeds for Robinhood.

## Getting a price feed for BtcUsd

```rust
use stylus_chainlink_price_feeds::{PriceFeed, get_latest_round_data_opt};

// This returns Some(x) if the call worked:
get_latest_round_data_bool(PriceFeed::BtcUsd).unwrap()
```
