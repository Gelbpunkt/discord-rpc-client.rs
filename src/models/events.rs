use super::shared::PartialUser;

use serde::{Deserialize, Serialize};

builder! {ReadyEvent
    v:      u32,
    config: RpcServerConfiguration,
    user:   PartialUser,
}

builder! {ErrorEvent
    code: u32,
    message: String,
}

builder! {RpcServerConfiguration
    cdn_host: String,
    api_endpoint: String,
    environment: String,
}
