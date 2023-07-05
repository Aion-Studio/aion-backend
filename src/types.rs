use std::future::Future;
use std::pin::Pin;

use prisma_client_rust::QueryError;

pub type QueryResult<T> = Result<T, QueryError>;

pub type AsyncResult<T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + Send>>;

pub type RepoFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, QueryError>> + Send + 'a>>;

