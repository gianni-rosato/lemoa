use lemmy_api_common::{
    lemmy_db_schema::{ListingType, SortType},
    lemmy_db_views::structs::PostView,
    post::{GetPosts, GetPostsResponse},
};

use crate::settings;

pub fn list_posts(
    page: i64,
    community_name: Option<String>,
    listing_type: Option<ListingType>,
    sort_type: Option<SortType>,
) -> std::result::Result<Vec<PostView>, reqwest::Error> {
    let params = GetPosts {
        page: Some(page),
        type_: listing_type,
        sort: sort_type,
        community_name,
        auth: settings::get_current_account().jwt,
        ..Default::default()
    };

    Ok(super::get::<GetPostsResponse, _>("/post/list", &params)?.posts)
}
