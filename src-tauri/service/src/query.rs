use ::entity::{artist, artist::Entity as Artist};
use ::entity::{album, album::Entity as Album};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_artist_by_id(db: &DbConn, id: i32) -> Result<Option<artist::Model>, DbErr> {
        Artist::find_by_id(id).one(db).await
    }

    pub async fn find_artist(db: &DbConn, name: &str) -> Result<Option<artist::Model>, DbErr> {
        Artist::find()
            .filter(artist::Column::Name.eq(name))
            .one(db)
            .await
    }

    pub async fn find_album_by_id(db: &DbConn, id: i32) -> Result<Option<album::Model>, DbErr> {
        Album::find_by_id(id).one(db).await
    }

    pub async fn find_album(db: &DbConn, name: &str, artist_id: i32) -> Result<Option<album::Model>, DbErr> {
        println!("{}", name);
        println!("{}", artist_id);
        Album::find()
            .filter(
                Condition::all()
                    .add(album::Column::Name.eq(name))
                    .add(album::Column::ArtistId.eq(artist_id))
            )
            .one(db)
            .await
    }
}