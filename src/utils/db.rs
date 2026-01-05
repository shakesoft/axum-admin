use rbatis::rbatis::RBatis;
use rbatis::rbdc::db::Connection;
use rbatis::rbdc::pool::{ConnectionManager, Pool};
use rbdc_mysql::MysqlDriver;
// use rbdc_mssql::MssqlDriver;
// use rbdc_pool_deadpool::DeadPool;
use rbdc_pool_fast::FastPool;

pub async fn init_db(url: &str) -> RBatis {
    let rb = RBatis::new();

    let manager = ConnectionManager::new(MysqlDriver {}, url).expect("create connection manager error");
    let pool = FastPool::new(manager).expect("create db pool error");
    // let pool = DeadPool::new(manager).expect("create db pool error");
    rb.init_pool(pool).expect("init db pool error");

    // 原先的注释里尝试直接访问 `rb.pool`（如 `rb.pool.get()`）会失败，因为 `RBatis` 内部的 pool 字段是私有的。
    // 下面提供一个正确的、独立的 MSSQL 连接测试函数，并展示如何直接使用 FastPool 获取连接并执行查询。
    // let uri = "Server=.; Database=Automobile8; Trusted_Connection=True;MultipleActiveResultSets=true;Encrypt=false;";
    // crate::utils::db::test_mssql_connection(uri).await.unwrap();
    rb
}

// pub async fn test_mssql_connection(uri: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     // Create a connection manager for MSSQL
//     let manager = ConnectionManager::new(MssqlDriver {}, uri)?;
//     // Create a fast pool
//     let pool = FastPool::new(manager)?;
//     // Get a connection from the pool (async)
//     let mut conn = pool.get().await?;
//     // Execute a simple query to verify connectivity
//     let rows = conn.get_rows("SELECT 1 as test", vec![]).await?;
//     println!("MSSQL test query rows: {:?}", rows);
//     Ok(())
// }

