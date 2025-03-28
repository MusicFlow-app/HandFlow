use serde::Deserialize;

#[derive(Deserialize)]
pub struct FavoriteRequest {
    pub increment: bool,
}