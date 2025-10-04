# Routes
POST /users 
POST /users/me 
GET /users/me 
PATCH /users/me 
GET /users/me/permisions/resource/{service_id}/{resource_type} 
GET /users/me/permissions/resource/{service_id}/{resource_type}/{resource_id}/{permission_name} 
POST /permissions/resource 
DELETE /permissions/resource/{service_id}/{resource_type}/{permission_name} 
POST /users/{user_id}/permissions/resource/{service_id}/{resource_type}/{resource_id}/{permission_name} 
DELETE /users/{user_id}/permissions/resource/{service_id}/{resource_type}/{resource_id}/{permission_name} 


# Use Cases
User Use Case:
+ `retrieve_from_token` +
+ login +
+ register +
+ `delete_as_admin` +
+ `delete_self` +
+ `update` +
+ `list_resource_permissions` +
+ `check_resource_permission` +
+ `grant_resource_permission` +
+ `revoke_resource_permission` +

Resource Permission Use Case:
+ `create` +
+ `delete` +

# Repositories

User Repository:
+ retrieve(login) -> Option<User>
+ insert(login, `password_hash`) -> Result<i32, E>
+ delete(id) -> Result<(), E>
+ update(`new_pwd`) -> Result<(), E>

Resource Permissions Repository:
+ retrieve(`service_id`, `resource_type`, `permission_name`) -> Option<ResourcePermission>
+ insert(`service_id`, `resource_type`, `permission_name`) -> Result<ResourcePermission, E>
+ delete(`service_id`, `resource_type`, `permission_name`) -> Result<(), E>

User Resource Permissions Repository:
+ list(`user_id`, `service_id`, `resource_type`) -> Vec<UserResourcePermission>
+ retrieve(`permission_id`, `user_id`, `resource_id`) -> Option<UserResourcePermission> // change to "id" instead
+ insert(`permission_id`, `user_id`, `resource_id`) -> Result<i32, E> 
+ delete(`permission_id`, `user_id`, `resource_id`) -> Result<(), E>

