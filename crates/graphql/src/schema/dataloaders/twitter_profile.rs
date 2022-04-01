use indexer_core::db::tables::twitter_handle_name_services;
use scalars::PublicKey;

use super::prelude::*;
use crate::schema::objects::twitter_profile::TwitterProfile;

#[async_trait]
impl TryBatchFn<PublicKey<TwitterProfile>, Option<TwitterProfile>> for Batcher {
    async fn load(
        &mut self,
        addresses: &[PublicKey<TwitterProfile>],
    ) -> TryBatchMap<PublicKey<TwitterProfile>, Option<TwitterProfile>> {
        let db_conn = self.db()?;

        let rows: Vec<models::TwitterHandle> = twitter_handle_name_services::table
            .select(twitter_handle_name_services::twitter_handle)
            .filter(twitter_handle_name_services::wallet_address.eq(any(addresses)))
            .limit(1)
            .load(&db_conn)
            .context("Failed to load twitter profile")?;

        //TODO: Create a TwitterProfile object from the twitter_handle and fetch images from the twitter api
        // let twitter_profile = TwitterProfile::new(rows[0].twitter_handle);

        // let twitter_profile_picture_response: TwitterProfilePictureResponse = http_client
        //     .get(format!(
        //         "https://api.twitter.com/2/users/by/username/{}",
        //         handle
        //     ))
        //     .header("Accept", "application/json")
        //     .query(&[("user.fields", "profile_image_url")])
        //     .bearer_auth(twitter_bearer_token)
        //     .send()
        //     .await
        //     .ok()?
        //     .json()
        //     .await
        //     .ok()?;

        Ok(rows.pop().map(Into::into).unwrap_or_default())
    }
}
