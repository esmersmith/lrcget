use entity::artist;
use sea_orm::{
    DatabaseBackend,
    DatabaseConnection,
    MockDatabase,
    MockExecResult
};

#[cfg(feature = "mock")]
pub fn prepare_mock_db() -> DatabaseConnection {
    MockDatabase::new(DatabaseBackend::Sqlite)
        .append_query_results([
            [artist::Model {
                id: 1,
                name: "Name A".to_owned(),
            }],
            [artist::Model {
                id: 5,
                name: "Name C".to_owned(),
            }],
            [artist::Model {
                id: 6,
                name: "Name D".to_owned(),
            }],
            [artist::Model {
                id: 1,
                name: "Name A".to_owned(),
            }],
            [artist::Model {
                id: 1,
                name: "New Name A".to_owned(),
            }],
            [artist::Model {
                id: 5,
                name: "Name C".to_owned(),
            }],
        ])
        .append_exec_results([
            MockExecResult {
                last_insert_id: 6,
                rows_affected: 1,
            },
            MockExecResult {
                last_insert_id: 6,
                rows_affected: 5,
            },
        ])
        .into_connection()
}