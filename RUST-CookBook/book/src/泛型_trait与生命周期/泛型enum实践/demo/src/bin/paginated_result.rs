#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

#[derive(Debug)]
struct Page<T> {
    items: Vec<T>,
    next_cursor: Option<String>,
}

#[derive(Debug)]
enum QueryError {
    InvalidCursor,
}

#[derive(Debug)]
enum PageResult<T, E> {
    Page(Page<T>),
    Failed(E),
}

fn query_users(cursor: Option<&str>) -> PageResult<User, QueryError> {
    match cursor {
        None => PageResult::Page(Page {
            items: vec![
                User {
                    id: 1,
                    name: "Alice".to_string(),
                },
                User {
                    id: 2,
                    name: "Bob".to_string(),
                },
            ],
            next_cursor: Some("page-2".to_string()),
        }),
        Some("page-2") => PageResult::Page(Page {
            items: vec![User {
                id: 3,
                name: "Carol".to_string(),
            }],
            next_cursor: None,
        }),
        Some(_) => PageResult::Failed(QueryError::InvalidCursor),
    }
}

fn main() {
    let mut cursor = None;

    loop {
        match query_users(cursor.as_deref()) {
            PageResult::Page(page) => {
                for user in page.items {
                    println!("{}: {}", user.id, user.name);
                }

                match page.next_cursor {
                    Some(next_cursor) => cursor = Some(next_cursor),
                    None => break,
                }
            }
            PageResult::Failed(error) => {
                eprintln!("query failed: {error:?}");
                break;
            }
        }
    }
}
