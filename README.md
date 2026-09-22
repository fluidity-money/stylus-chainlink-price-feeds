# Stylus Chainlink Price Feeds

Chainlink Price Feeds for Arbitrum and Robinhood.

## Getting a raw price (using stylus-sdk)

```rust
use stylus_chainlink_price_feeds::{arbitrum::Arbitrum, get_latest_round_data};

let price = get_latest_round_data(self.vm(), Call::new(), Arbitrum::BtcUsd)?;
```

## Getting whole and fractional words

The split helpers divide the raw Chainlink answer at the feed's declared decimal
precision. For example, an 8-decimal raw answer of `123000004` becomes `(1,
23000004)`. The fractional word is numeric rather than zero-padded; use the
feed's `decimals()` value when formatting it for display.

With stylus-sdk:

```rust
use stylus_chainlink_price_feeds::{arbitrum::Arbitrum, get_latest_round_data_split};

let (whole, fractional) =
    get_latest_round_data_split(self.vm(), Call::new(), Arbitrum::BtcUsd)?;
```

With bobcat-sdk:

```rust
use stylus_chainlink_price_feeds::{
    arbitrum::Arbitrum,
    get_latest_round_data_split_bool,
    get_latest_round_data_split_opt,
};

let (success, (whole, fractional)) =
    get_latest_round_data_split_bool(Arbitrum::BtcUsd);

let price = get_latest_round_data_split_opt(Arbitrum::BtcUsd);
```

## Getting a historical price

With stylus-sdk:

```rust
use stylus_chainlink_price_feeds::{arbitrum::Arbitrum, get_price_at};

let price = get_price_at(
    self.vm(),
    Call::new(),
    Arbitrum::BtcUsd,
    timestamp,
)?;
```

With bobcat-sdk:

```rust
use stylus_chainlink_price_feeds::{arbitrum::Arbitrum, get_price_at_opt};

let price = get_price_at_opt(Arbitrum::BtcUsd, timestamp);
```
