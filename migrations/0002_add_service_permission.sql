CREATE TABLE service_permissions (
  id SERIAL PRIMARY KEY,
  service_id TEXT NOT NULL UNIQUE
);

CREATE TABLE user_service_permissions (
  service_permission_id INTEGER NOT NULL REFERENCES service_permissions(id) ON DELETE CASCADE,
  user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  UNIQUE (service_permission_id, user_id)
);
