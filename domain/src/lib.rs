pub mod models;
pub use models::{
    Permission, 
    Group,
    User,
    Claims
};

pub mod params;
pub use params::{
    // permission params
    PermissionCreateParams,
    PermissionCreateParamsBuilder,
    
    PermissionDeleteParams,
    PermissionDeleteParamsBuilder,
    
    // group params
    GroupCreateParams,
    GroupCreateParamsBuilder,
    
    GroupDeleteParams,
    GroupDeleteParamsBuilder,
    
    GroupGrantPermissionParams,
    GroupGrantPermissionParamsBuilder,
    
    GroupRevokePermissionParams,
    GroupRevokePermissionParamsBuilder,
    
    // user params
    UserLoginParams,
    UserLoginParamsBuilder,
    
    UserRegisterParams,
    UserRegisterParamsBuilder,
    
    UserInfoParams,
    UserInfoParamsBuilder,
    
    UserUpdatePwdParams,
    UserUpdatePwdParamsBuilder,
    
    UserAuthorizeParams,
    UserAuthorizeParamsBuilder,
    
    UserGrantGroupParams,
    UserGrantGroupParamsBuilder,
    
    UserRevokeGroupParams,
    UserRevokeGroupParamsBuilder,
};

pub mod errors;
pub use errors::{
    // permission errors
    PermissionCreateError,
    PermissionDeleteError,

    // group errors
    GroupCreateError,
    GroupDeleteError,
    GroupGrantPermissionError,
    GroupRevokePermissionError,
    
    // user errors
    UserInfoError,
    UserAuthorizeError,
    UserLoginError,
    UserUpdatePwdError,
    UserRegisterError,
    UserGrantGroupError,
    UserRevokeGroupError
};
