use serde::{Deserialize, Serialize};

builder! {PartialUser
    id:            String,
    username:      String,
    discriminator: String,
    avatar:        String,
}

impl PartialUser {
    pub fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn get_username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    pub fn get_discriminator(&self) -> Option<&str> {
        self.discriminator.as_deref()
    }

    pub fn get_avatar(&self) -> Option<&str> {
        self.avatar.as_deref()
    }
}
