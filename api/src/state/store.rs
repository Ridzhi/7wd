use crate::{
    prelude::*,
    account::{
        model::*
    }
};

use std::sync::Arc;
use anyhow::{Result};
use deadpool_postgres::Pool;
use sea_query::*;
use tokio_postgres::Row;

#[enum_def(table_name = "user")]
struct UserRecord {
    pub id: i32,
    pub nickname: String,
    pub rating: i16,
    pub email: String,
    pub password: String,
    pub settings: serde_json::Value,
    pub created_at: time::PrimitiveDateTime,
}

impl UserRecord {
    pub fn values_skip_id(self) -> Vec<SimpleExpr> {
        self.values().into_iter().skip(1).collect()
    }

    pub fn values(self) -> Vec<SimpleExpr> {
        vec![
            self.id.into(),
            self.nickname.into(),
            self.rating.into(),
            self.email.into(),
            self.password.into(),
            self.settings.into(),
            self.created_at.into(),
        ]
    }
}

impl From<User> for UserRecord {
    fn from(value: User) -> Self {
        UserRecord {
            id: value.id as i32,
            nickname: value.nickname,
            rating:  value.rating as i16,
            email: value.email,
            password: value.password,
            settings: serde_json::to_value(value.settings).unwrap(),
            created_at: value.created_at.0,
        }
    }
}

impl From<Row> for User {
    fn from(value: Row) -> Self {
        Self {
            id: {
                let id: i32 = value.get(UserRecordIden::Id.as_str());
                id as u32
            },
            nickname: value.get(UserRecordIden::Nickname.as_str()),
            rating:  {
                let rating: i16 = value.get(UserRecordIden::Rating.as_str());
                rating as u16
            },
            email: value.get(UserRecordIden::Email.as_str()),
            password: value.get(UserRecordIden::Password.as_str()),
            settings: serde_json::from_value(value.get(UserRecordIden::Settings.as_str())).unwrap(),
            created_at: UtcDateTime(value.get(UserRecordIden::CreatedAt.as_str())),
        }
    }
}

pub struct UserStore {
    pg: Arc<Pool>,
}

impl UserStore {
    pub fn new(pg: Arc<Pool>) -> Self {
        Self {
            pg,
        }
    }

    pub async fn save(&self, u: User) -> Result<User> {
        let q = Query::insert()
            .into_table(UserRecordIden::Table)
            .columns([
                UserRecordIden::Nickname,
                UserRecordIden::Rating,
                UserRecordIden::Email,
                UserRecordIden::Password,
                UserRecordIden::Settings,
                UserRecordIden::CreatedAt,
            ])
            .values(UserRecord::from(u).values_skip_id())?
            // .values_panic(Record::from(u).values_skip_id())
            .returning_all()
            .to_owned();

        let sql = q.to_string(PostgresQueryBuilder);

        let conn = self.pg.get().await?;

        Ok(conn.query_one(&sql, &[]).await?.into())
    }

    pub async fn find(&self, id: &UserId) -> Result<Option<User>> {
        self.find_with(UserOptions::default().with_id(*id)).await
    }
    
    pub async fn find_with(&self, o: &UserOptions) -> Result<Option<User>> {
        Ok(self.find_many(o).await?.first().cloned())
    }

    pub async fn find_many(&self, o: &UserOptions) -> Result<Vec<User>> {
        let mut q = Query::select()
            .from(UserRecordIden::Table)
            .columns([
                UserRecordIden::Id,
                UserRecordIden::Nickname,
                UserRecordIden::Rating,
                UserRecordIden::Email,
                UserRecordIden::Password,
                UserRecordIden::Settings,
                UserRecordIden::CreatedAt,
            ])
            .to_owned();

        if let Some(id) = o.id.as_ref() {
            q.and_where(Expr::col(UserRecordIden::Id).eq(*id as i64));
        }

        if let Some(email) = o.email.as_ref() {
            q.and_where(Expr::col(UserRecordIden::Email).eq(email.as_str()));
        }

        if let Some(nickname) = o.nickname.as_ref() {
            q.and_where(Expr::col(UserRecordIden::Nickname).eq(nickname.as_str()));
        }

        let sql = q.to_string(PostgresQueryBuilder);
        let conn = self.pg.get().await?;

        let rows = conn.query(&sql, &[]).await?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }
}