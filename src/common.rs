use uuid::Uuid;

/// UUIDv7: time-ordered, so IDs sort chronologically and stay
/// index-friendly in whatever store ends up backing this (unlike v4's
/// random distribution).
pub type Id = Uuid;

pub fn new_id() -> Id {
    Uuid::now_v7()
}

pub type Timestamp = time::OffsetDateTime;
