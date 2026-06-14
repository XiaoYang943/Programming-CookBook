#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

#[derive(Debug)]
enum ApiError {
    NotFound { resource: String },
    Unavailable,
}

fn fetch_user(id: u64) -> Result<User, ApiError> {
    match id {
        1 => Ok(User {
            id,
            name: "Alice".to_string(),
        }),
        503 => Err(ApiError::Unavailable),
        _ => Err(ApiError::NotFound {
            resource: format!("user/{id}"),
        }),
    }
}

fn describe_user(id: u64) -> String {
    match fetch_user(id) {
        Ok(user) => format!("loaded user {}: {}", user.id, user.name),
        Err(ApiError::NotFound { resource }) => format!("{resource} was not found"),
        Err(ApiError::Unavailable) => "service is temporarily unavailable".to_string(),
    }
}

fn main() {
    println!("{}", describe_user(1));
    println!("{}", describe_user(2));
    println!("{}", describe_user(503));
}
