use crate::model::value_objects::{CreatedAt, CreatedBy, UpdatedAt, UpdatedBy};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Audit {
    pub created_at: CreatedAt,
    pub updated_at: Option<UpdatedAt>,
    pub created_by: Option<CreatedBy>,
    pub updated_by: Option<UpdatedBy>,
}

impl Audit {
    pub fn with_created_at(created_at: CreatedAt) -> Self {
        Self {
            created_at,
            updated_at: None,
            created_by: None,
            updated_by: None,
        }
    }
}
