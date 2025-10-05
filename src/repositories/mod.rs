pub mod user;
pub use user::UserRepository;

pub mod resource_permission;
pub use resource_permission::ResourcePermissionRepository;

pub mod user_resource_permission;
pub use user_resource_permission::UserResourcePermissionRepository;

pub const PAGE_SIZE: u8 = 5;
