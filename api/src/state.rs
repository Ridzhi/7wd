pub mod config;
mod password;
mod user_repo;

use std::sync::{Arc, OnceLock};
use deadpool::Runtime;
use self::{config::Config, password::Password, user_repo::UserRepoImpl};
use deadpool_postgres::Pool;
use redis::Client as RedisClient;

#[derive(Clone)]
pub struct AppState {
    config: Factory<Arc<Config>>,
    pg: Factory<Arc<Pool>>,
    redis: Factory<Arc<RedisClient>>,
    passwd: Factory<Arc<Password>>,
    user_repo: Factory<Arc<UserRepoImpl>>,
}

impl AppState {
    pub fn config(&self) -> Arc<Config> {
        self.resolve(&self.config)
    }

    pub fn pg(&self) -> Arc<Pool> {
        self.resolve(&self.pg)
    }

    pub fn rds(&self) -> Arc<RedisClient> {
        self.resolve(&self.redis)
    }

    pub fn passwd(&self) -> Arc<Password> {
        self.resolve(&self.passwd)
    }

    pub fn user_repo(&self) -> Arc<UserRepoImpl> {
        self.resolve(&self.user_repo)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            config: Factory::once(|_| Arc::new(Config::from_env())),
            pg: Factory::once(|state| {
                let pg = &state.config().pg;
                let pool = pg.create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls).unwrap();
                // let _ = pool.get().await.expect("pg connection failed");
                // let _ = pool.get();
                Arc::new(pool)
            }),
            redis: Factory::once(|state| {
                let dsn = format!(
                    "redis://:{}@localhost:{}",
                    state.config().redis.password,
                    state.config().redis.port,
                );

                Arc::new(RedisClient::open(dsn).unwrap())
            }),
            passwd: Factory::once(|_state| Arc::new(Password)),
            user_repo: Factory::once(|state| Arc::new(UserRepoImpl::new(state.pg()))),
        }
    }
}

impl AppState {
    fn resolve<T>(&self, factory: &Factory<T>) -> T
    where
        T: Clone,
    {
        (factory.0)(self)
    }
}

#[derive(Clone)]
struct Factory<T>(Arc<dyn Fn(&AppState) -> T + Send + Sync>);

impl<T> Factory<T> {
    pub fn once(f: impl Fn(&AppState) -> T + Send + Sync + 'static) -> Self
    where
        T: Send + Sync + Clone + 'static,
    {
        let cell = OnceLock::new();
        Factory(Arc::new(move |di| cell.get_or_init(|| f(di)).clone()))
    }
}