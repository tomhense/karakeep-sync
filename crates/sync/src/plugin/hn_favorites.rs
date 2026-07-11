use crate::settings;
use async_trait::async_trait;
use futures::{Stream, StreamExt};
use hnscraper::{stream_favorited_submissions, username_from_auth};
use karakeep_client::BookmarkCreate;
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct HNFavorited {}

#[async_trait]
impl super::Plugin for HNFavorited {
    fn list_name(&self) -> &'static str {
        "HN Favorited"
    }

    async fn to_bookmark_stream(
        &self,
    ) -> anyhow::Result<Pin<Box<dyn Stream<Item = Vec<BookmarkCreate>> + Send>>> {
        let settings = settings::get_settings();
        let auth = settings
            .hn
            .auth
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("HN auth token is not set"))?;

        let username = username_from_auth(auth)
            .ok_or_else(|| anyhow::anyhow!("Failed to extract username from auth token"))?;

        let stream = stream_favorited_submissions(auth, &username)?.map(|page| {
            page.into_iter()
                .map(|post| BookmarkCreate {
                    title: post.title,
                    url: post.url,
                    // HN does not provide timestamp for when the post was favorited
                    created_at: None,
                })
                .collect::<Vec<_>>()
        });

        Ok(Box::pin(stream))
    }

    fn is_activated(&self) -> bool {
        let settings = &settings::get_settings();
        settings.hn.auth.is_some() && !settings.hn.auth.as_ref().unwrap().is_empty()
    }

    fn recurring_schedule(&self) -> String {
        let settings = &settings::get_settings();
        settings
            .hn
            .favorited_schedule
            .as_ref()
            .filter(|schedule| !schedule.is_empty())
            .cloned()
            .unwrap_or_else(|| settings.hn.schedule.clone())
    }
}
