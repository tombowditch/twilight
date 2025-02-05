use crate::{
    client::Client,
    request::Request,
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{guild::GuildIntegration, id::GuildId};

/// Get the guild's integrations.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildIntegrations<'a> {
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildIntegrations<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self { guild_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<GuildIntegration>> {
        let request = Request::from_route(&Route::GetGuildIntegrations {
            guild_id: self.guild_id.0,
        });

        self.http.request(request)
    }
}
