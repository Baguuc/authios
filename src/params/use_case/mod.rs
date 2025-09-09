pub mod permission;
pub use permission::{
    // create params
    PermissionCreateParams,
    PermissionCreateParamsBuilder,
    // delete params
    PermissionDeleteParams,
    PermissionDeleteParamsBuilder,
};

pub mod group;
pub use group::{
    // create params
    GroupCreateParams,
    GroupCreateParamsBuilder,
    // delete params
    GroupDeleteParams,
    GroupDeleteParamsBuilder,
    // grant permission params
    GroupGrantPermissionParams,
    GroupGrantPermissionParamsBuilder,
    // revoke permission params 
    GroupRevokePermissionParams,
    GroupRevokePermissionParamsBuilder,
};

pub mod user;
pub use user::{
    // register params
    UserRegisterParams,
    UserRegisterParamsBuilder,
    // login params
    UserLoginParams,
    UserLoginParamsBuilder,
    // authorize params
    UserAuthorizeParams,
    UserAuthorizeParamsBuilder,
    // info params
    UserInfoParams,
    UserInfoParamsBuilder,
    // update pwd params
    UserUpdatePwdParams,
    UserUpdatePwdParamsBuilder,
    // grant group params
    UserGrantGroupParams,
    UserGrantGroupParamsBuilder,
    // revoke group params 
    UserRevokeGroupParams,
    UserRevokeGroupParamsBuilder,
    // init root params 
    UserInitRootParams,
    UserInitRootParamsBuilder,
    // list permissions params
    UserListPermissionsParams,
    UserListPermissionsParamsBuilder
};
