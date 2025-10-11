pub mod user;
pub use user::UserRepository;

pub mod resource_permission;
pub use resource_permission::ResourcePermissionRepository;

pub mod service_permission;
pub use service_permission::ServicePermissionRepository;

pub mod user_resource_permission;
pub use user_resource_permission::UserResourcePermissionRepository;

pub mod user_service_permission;
pub use user_service_permission::UserServicePermissionRepository;

pub const PAGE_SIZE: u8 = 5;
