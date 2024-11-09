use crate::{
    prelude::*,
    account::{
        model::*
    }
};

use std::sync::Arc;
use anyhow::{bail, Result};
use deadpool_postgres::Pool;
use sea_query::*;
use tokio_postgres::Row;

#[enum_def(table_name = "user")]
struct Record {
    pub id: SimpleExpr,
    pub nickname: SimpleExpr,
    pub email: SimpleExpr,
    pub password: SimpleExpr,
    pub settings: SimpleExpr,
    pub created_at: SimpleExpr,
}

impl Record {
    pub fn values_skip_id(self) -> Vec<SimpleExpr> {
        self.values().into_iter().skip(1).collect()
    }

    pub fn values(self) -> Vec<SimpleExpr> {
        vec![
            self.id,
            self.nickname,
            self.email,
            self.password,
            self.settings,
            self.created_at,
        ]
    }
}

impl From<User> for Record {
    fn from(value: User) -> Self {
        Record {
            id: SimpleExpr::Value(
                value.id.0.into(),
            ),
            nickname: SimpleExpr::Value(
                value.nickname.0.into(),
            ),
            email: SimpleExpr::Value(
                value.email.0.into()
            ),
            password: SimpleExpr::Value(
                value.password.into(),
            ),
            settings: SimpleExpr::Value(
                serde_json::to_string(&value.settings).unwrap().into(),
            ),
            created_at: SimpleExpr::Value(
                value.created_at.0.into()
            ),
        }
    }
}

impl From<Row> for User {
    fn from(value: Row) -> Self {
        Self {
            id: UserId(value.get(RecordIden::Id.to_string().as_str())),
            nickname: Nickname(value.get(RecordIden::Nickname.to_string().as_str())),
            email: Email(value.get(RecordIden::Email.to_string().as_str())),
            password: value.get(RecordIden::Password.to_string().as_str()),
            settings: serde_json::from_value(value.get(RecordIden::Settings.to_string().as_str())).unwrap(),
            created_at: UtcDateTime(value.get(RecordIden::CreatedAt.to_string().as_str())),
        }
    }
}

pub struct UserRepoImpl {
    pg: Arc<Pool>,
}

impl UserRepoImpl {
    pub fn new(pg: Arc<Pool>) -> Self {
        Self {
            pg,
        }
    }

    pub async fn save(&self, u: User) -> Result<User> {
        let q = Query::insert()
            .into_table(RecordIden::Table)
            .columns([
                RecordIden::Nickname,
                RecordIden::Email,
                RecordIden::Password,
                RecordIden::Settings,
                RecordIden::CreatedAt,
            ])
            .values_panic(Record::from(u.clone()).values_skip_id())
            .returning_all()
            .to_owned();

        let sql = q.to_string(PostgresQueryBuilder);

        let conn = self.pg.get().await?;

        Ok(conn.query_one(&sql, &[]).await?.into())
    }

    pub async fn find(&self, o: &UserOptions) -> Result<User> {
        let users = self.find_many(o).await?;

        if users.is_empty() {
            return bail!(ErrorKind::UserNotFound);
        }

        Ok(users.first().unwrap().clone())
    }

    pub async fn find_many(&self, o: &UserOptions) -> Result<Vec<User>> {
        let mut q = Query::select()
            .from(RecordIden::Table)
            .columns([
                RecordIden::Id,
                RecordIden::Nickname,
                RecordIden::Email,
                RecordIden::Password,
                RecordIden::Settings,
                RecordIden::CreatedAt,
            ])
            .to_owned();

        if let Some(id) = o.id.as_ref() {
            q.and_where(Expr::col(RecordIden::Id).eq(id.0));
        }

        if let Some(email) = o.email.as_ref() {
            q.and_where(Expr::col(RecordIden::Email).eq(email.0.as_str()));
        }

        if let Some(nickname) = o.nickname.as_ref() {
            q.and_where(Expr::col(RecordIden::Nickname).eq(nickname.0.as_str()));
        }

        let sql = q.to_string(PostgresQueryBuilder);
        let conn = self.pg.get().await?;

        let rows = conn.query(&sql, &[]).await?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }
}