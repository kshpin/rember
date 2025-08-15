pub mod notes;
pub mod service;
pub mod tags;

use notes::NotesRepository;
use sqlx::PgPool;
use tags::TagsRepository;

#[derive(Clone)]
pub struct Database {
    pub notes: NotesRepository,
    pub tags: TagsRepository,
}

impl Database {
    pub fn with_pool(pool: PgPool) -> Self {
        Self {
            notes: NotesRepository::new(pool.clone()),
            tags: TagsRepository::new(pool.clone()),
        }
    }
}
