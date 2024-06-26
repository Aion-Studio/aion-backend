use crate::prisma::PrismaClient;
use once_cell::sync::OnceCell;
use std::sync::Arc;

use lazy_static::lazy_static;

lazy_static! {
    static ref DB_CLIENT: OnceCell<Arc<PrismaClient>> = OnceCell::new();
}

pub fn get_db_client() -> Arc<PrismaClient> {
    DB_CLIENT
        .get()
        .expect("Database client not initialized")
        .clone()
}

pub async fn initialize_db(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = PrismaClient::_builder().with_url(url).build().await?;

    DB_CLIENT
        .set(Arc::new(client))
        .map_err(|_| "Database client already initialized")?;
    Ok(())
}

#[macro_export]
macro_rules! db {
    () => {{
        fn get_db_client_inner() -> ::std::sync::Arc<$crate::prisma::PrismaClient> {
            $crate::get_db_client()
        }
        get_db_client_inner()
    }};
    ($($tt:tt)*) => {{
        fn get_db_client_inner() -> ::std::sync::Arc<$crate::prisma::PrismaClient> {
            $crate::get_db_client()
        }
        get_db_client_inner().$($tt)*
    }};
}
