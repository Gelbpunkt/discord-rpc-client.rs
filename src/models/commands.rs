use super::shared::PartialUser;

use serde::{Deserialize, Serialize};

builder! {SubscriptionArgs
    secret: String,     // Activity{Join,Spectate}
    user: PartialUser,  // ActivityJoinRequest
}

builder! {Subscription
    evt: String,
}
