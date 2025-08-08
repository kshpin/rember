pub mod notes;

use notes::NoteRepository;
use sqlx::PgPool;

#[derive(Clone)]
pub struct Database {
    pub notes: NoteRepository,
}

impl Database {
    pub fn with_pool(pool: PgPool) -> Self {
        Self {
            notes: NoteRepository::new(pool.clone()),
        }
    }
}
