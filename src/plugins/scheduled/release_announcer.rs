use std::sync::Arc;
use serenity::client::Context;

use crate::constants::channels;

pub async fn task_game_release_announcement_sentry(ctx: Arc<Context>) -> () {
    loop {
        
        tokio::time::sleep(channels::release_channels::RELEASE_CHANNELS_CHECK_INTERVAL).await;
    }
}

async fn fetch_releases(ctx: Arc<Context>) -> () {
    
}
