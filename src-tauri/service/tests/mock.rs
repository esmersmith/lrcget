mod prepare;

use service::{Mutation, Query};
use entity::artist;
use prepare::prepare_mock_db;

#[tokio::test]
async fn main() {
    let db = &prepare_mock_db();

    {
        let artist = Query::find_artist_by_id(db, 1).await.unwrap().unwrap();

        assert_eq!(artist.id, 1);
    }

    {
        let artist = Query::find_artist_by_id(db, 5).await.unwrap().unwrap();

        assert_eq!(artist.id, 5);
    }

    {
        let artist = Mutation::add_artist(
            db,
            artist::Model {
                id: 0,
                name: "Name D".to_owned(),
            },
        )
        .await
        .unwrap();

        assert_eq!(
            artist,
            artist::ActiveModel {
                id: sea_orm::ActiveValue::Unchanged(6),
                name: sea_orm::ActiveValue::Unchanged("Name D".to_owned())
            }
        );
    }

    {
        let artist = Mutation::update_artist_by_id(
            db,
            1,
            artist::Model {
                id: 1,
                name: "New Name A".to_owned(),
            },
        )
        .await
        .unwrap();

        assert_eq!(
            artist,
            artist::Model {
                id: 1,
                name: "New Name A".to_owned(),
            }
        );
    }
}