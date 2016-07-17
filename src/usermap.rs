use std::collections::HashMap;

/// Key: username
/// Value: id of chatroom user is currently in
///        0 if not in a chatroom
pub type UserMap = HashMap<String, usize>;