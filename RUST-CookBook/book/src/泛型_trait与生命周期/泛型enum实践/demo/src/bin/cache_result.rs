#[derive(Debug)]
struct Product {
    id: u64,
    name: String,
}

#[derive(Debug)]
enum CacheResult<T> {
    Hit(T),
    Miss,
    Expired(T),
}

fn find_product(id: u64) -> CacheResult<Product> {
    match id {
        1 => CacheResult::Hit(Product {
            id,
            name: "Keyboard".to_string(),
        }),
        2 => CacheResult::Expired(Product {
            id,
            name: "Monitor".to_string(),
        }),
        _ => CacheResult::Miss,
    }
}

fn load_product(id: u64) -> Product {
    match find_product(id) {
        CacheResult::Hit(product) => product,
        CacheResult::Expired(product) => {
            println!("return stale value and schedule refresh for product {id}");
            product
        }
        CacheResult::Miss => {
            println!("load product {id} from database and populate cache");
            Product {
                id,
                name: "Database product".to_string(),
            }
        }
    }
}

fn main() {
    for id in 1..=3 {
        let product = load_product(id);
        println!("{}: {}", product.id, product.name);
    }
}
