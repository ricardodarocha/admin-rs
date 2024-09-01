use uuid::Uuid;
pub enum UuidKind {
    V4,
    V7,
}

use UuidKind::*;

pub fn generate_uuid(kind: UuidKind) -> String  {
    match kind {
        V7 => Uuid::now_v7().to_string(),
        V4 => Uuid::new_v4().to_string(),
    }
}