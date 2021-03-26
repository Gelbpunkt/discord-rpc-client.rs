use serde::{Deserialize, Serialize};

builder! {PartialUser
    id:            String,
    username:      String,
    discriminator: String,
    avatar:        String,
}
