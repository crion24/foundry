use clap::{builder::RangedU64ValueParser, Parser};
use foundry_utils::Retry;

/// Retry config used when waiting for verification
pub const RETRY_CHECK_ON_VERIFY: RetryArgs = RetryArgs { retries: 8, delay: 15 };

/// Retry config used when waiting for a created contract
pub const RETRY_VERIFY_ON_CREATE: RetryArgs = RetryArgs { retries: 15, delay: 5 };

/// Retry arguments for contract verification.
#[derive(Debug, Clone, Copy, Parser)]
#[clap(about = "Allows to use retry arguments for contract verification")] // override doc
pub struct RetryArgs {
    #[clap(
        long,
        help = "Number of attempts for retrying verification",
        default_value = "5",
        value_parser = RangedU64ValueParser::<u32>::new().range(1..=10),
        value_name = "RETRIES"
    )]
    pub retries: u32,

    #[clap(
        long,
        help = "Optional delay to apply inbetween verification attempts in seconds.",
        default_value = "5",
        value_parser = RangedU64ValueParser::<u32>::new().range(0..=30),
        value_name = "DELAY"
    )]
    pub delay: u32,
}

impl Default for RetryArgs {
    fn default() -> Self {
        RETRY_VERIFY_ON_CREATE
    }
}

impl From<RetryArgs> for Retry {
    fn from(r: RetryArgs) -> Self {
        Retry::new(r.retries, Some(r.delay))
    }
}
