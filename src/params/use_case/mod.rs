pub mod permission;
pub use permission::{
    PermissionCreateParams,
    PermissionDeleteParams,
};

pub mod group;
pub use group::{
    GroupCreateParams,
    GroupDeleteParams,
    GroupGrantPermissionParams,
    GroupRevokePermissionParams,
};

pub mod user;
pub use user::{
    UserRegisterParams,
    UserLoginParams,
    UserAuthorizeParams,
    UserInfoParams,
    UserUpdatePwdParams,
    UserGrantGroupParams,
    UserRevokeGroupParams,
    UserInitRootParams,
    UserListPermissionsParams,
};
