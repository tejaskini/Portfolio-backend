use crate::models::auth::AuthenticatedUser;

pub fn check_permission(user: &AuthenticatedUser, required_permission: &str) -> bool {
    user.permissions.contains(&required_permission.to_string())
}