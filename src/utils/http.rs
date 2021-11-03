//! Utilities related to HTTP requests

use ureq::{Agent, AgentBuilder};

/// Get an HTTP agent, for making requests. This should be used for *all* HTTP
/// requests, because it provides important configuration on the agent.
pub fn agent() -> Agent {
    AgentBuilder::new()
        // The OSRS Wiki requests we set this for any requests to their API, but
        // we might as well just put it on all requests for consistency
        // https://oldschool.runescape.wiki/w/RuneScape:Real-time_Prices#Please_set_a_descriptive_User-Agent!
        .user_agent(&format!("osrs-cli/{}", env!("CARGO_PKG_VERSION")))
        .build()
}
