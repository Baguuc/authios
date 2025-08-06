pub mod repositories;
pub mod use_cases;
pub mod utils;

use repositories::{
    PermissionsRepository,
    GroupsRepository,
    GroupPermissionsRepository,
    UsersRepository,
    UserGroupsRepository
};
pub use use_cases::{
    PermissionsUseCase,
    GroupsUseCase,
    UsersUseCase,
};

const MIGRATIONS: [&str; 10] = [
"CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  login VARCHAR(32) NOT NULL UNIQUE,
  pwd VARCHAR(97) NOT NULL
);",

"CREATE INDEX IF NOT EXISTS users_login_index ON users USING HASH (login);",

"CREATE TABLE IF NOT EXISTS groups (name TEXT PRIMARY KEY);",

"CREATE TABLE IF NOT EXISTS user_groups (
  user_login TEXT NOT NULL,
  group_name TEXT NOT NULL,

  FOREIGN KEY (user_login) REFERENCES users(login) ON DELETE CASCADE,
  FOREIGN KEY (group_name) REFERENCES groups(name) ON DELETE CASCADE
);",

"CREATE TABLE IF NOT EXISTS permissions (name TEXT PRIMARY KEY);",

"CREATE TABLE IF NOT EXISTS group_permissions (
  group_name TEXT NOT NULL,
  permission_name TEXT NOT NULL,

  FOREIGN KEY (group_name) REFERENCES groups(name) ON DELETE CASCADE,
  FOREIGN KEY (permission_name) REFERENCES permissions(name) ON DELETE CASCADE
);",

"ALTER TABLE group_permissions DROP CONSTRAINT IF EXISTS group_permissions_pair_unique;",

"ALTER TABLE group_permissions ADD CONSTRAINT group_permissions_pair_unique UNIQUE (group_name, permission_name);",

"ALTER TABLE user_groups DROP CONSTRAINT IF EXISTS user_groups_pair_unique;",

"ALTER TABLE user_groups ADD CONSTRAINT user_groups_pair_unique UNIQUE (user_login, group_name);"
];
