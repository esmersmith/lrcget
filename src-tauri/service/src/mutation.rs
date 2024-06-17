use ::entity::{artist, artist::Entity as Artist};
use ::entity::album;
use ::entity::track;
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn add_album(
        db: &DbConn,
        form_data: album::Model,
    ) -> Result<album::ActiveModel, DbErr> {
        album::ActiveModel {
            name: Set(form_data.name.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn add_artist(
        db: &DbConn,
        form_data: artist::Model,
    ) -> Result<artist::ActiveModel, DbErr> {
        artist::ActiveModel {
            name: Set(form_data.name.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn add_track(
        db: &DbConn,
        form_data: track::Model,
    ) -> Result<track::ActiveModel, DbErr> {
        track::ActiveModel {
            title: Set(form_data.title.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_artist_by_id(
        db: &DbConn,
        id: i32,
        form_data: artist::Model,
    ) -> Result<artist::Model, DbErr> {
        let artist: artist::ActiveModel = Artist::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find artist.".to_owned()))
            .map(Into::into)?;

        artist::ActiveModel {
            id: artist.id,
            name: Set(form_data.name.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_artist(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let artist: artist::ActiveModel = Artist::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find artist.".to_owned()))
            .map(Into::into)?;

        artist.delete(db).await
    }

    pub async fn delete_all_artists(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Artist::delete_many().exec(db).await
    }
}