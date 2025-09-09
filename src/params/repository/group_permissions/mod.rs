pub mod insert;
pub use insert::{
    InsertParams as GroupPermissionInsertParams,
    InsertParamsBuilder as GroupPermissionInsertParamsBuilder
};

pub mod delete;
pub use delete::{
    DeleteParams as GroupPermissionDeleteParams,
    DeleteParamsBuilder as GroupPermissionDeleteParamsBuilder
};
