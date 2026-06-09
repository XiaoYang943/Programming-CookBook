use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct DatabaseConnection {
    id: usize,
}

impl DatabaseConnection {
    fn execute(&self, sql: &str) {
        println!("connection {} executes: {sql}", self.id);
    }
}

#[derive(Clone)]
struct ConnectionPool {
    available: Rc<RefCell<Vec<DatabaseConnection>>>,
}

impl ConnectionPool {
    fn new(size: usize) -> Self {
        let connections = (0..size)
            .map(|id| DatabaseConnection { id })
            .collect();

        Self {
            available: Rc::new(RefCell::new(connections)),
        }
    }

    fn acquire(&self) -> PooledConnection {
        let connection = self
            .available
            .borrow_mut()
            .pop()
            .expect("no available database connection");

        PooledConnection {
            connection: Some(connection),
            pool: self.clone(),
        }
    }

    fn available_count(&self) -> usize {
        self.available.borrow().len()
    }
}

struct PooledConnection {
    // 使用 Option，便于在 Drop 中安全地取出连接并归还连接池。
    connection: Option<DatabaseConnection>,
    pool: ConnectionPool,
}

impl PooledConnection {
    fn execute(&self, sql: &str) {
        self.connection
            .as_ref()
            .expect("connection already returned")
            .execute(sql);
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        if let Some(connection) = self.connection.take() {
            println!("return connection {} to pool", connection.id);
            self.pool.available.borrow_mut().push(connection);
        }
    }
}

fn main() {
    let pool = ConnectionPool::new(1);

    {
        let connection = pool.acquire();
        assert_eq!(pool.available_count(), 0);
        connection.execute("SELECT * FROM users");
    } // connection 离开作用域，Drop 自动把数据库连接归还连接池。

    assert_eq!(pool.available_count(), 1);
}
