pub mod insert;
pub use insert::{
    GroupPermissionInsertParams,
    InsertParamsBuilder as GroupPermissionInsertParamsBuilder
};

pub mod delete;
pub use delete::{
    GroupPermissionDeleteParams,
    DeleteParamsBuilder as GroupPermissionDeleteParamsBuilder
};
