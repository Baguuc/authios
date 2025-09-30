CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  login TEXT NOT NULL,
  password_hash TEXT NOT NULL,
  UNIQUE (login)
);

CREATE TABLE resource_permissions (
  id SERIAL PRIMARY KEY,
  service_id TEXT NOT NULL,
  resource_type TEXT NOT NULL,
  permission_name TEXT NOT NULL,
  UNIQUE (service_id, resource_type, permission_name)
);

CREATE TABLE user_resource_permissions (
  resource_permission_id INTEGER NOT NULL REFERENCES resource_permissions(id) ON DELETE CASCADE,
  resource_id INTEGER NOT NULL, 
  user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  UNIQUE (resource_permission_id, resource_id, user_id)
);
