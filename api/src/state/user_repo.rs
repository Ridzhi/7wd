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
    pub id: i64,
    pub nickname: String,
    pub rating: i16,
    pub email: String,
    pub password: String,
    pub settings: serde_json::Value,
    pub created_at: time::PrimitiveDateTime,
}

impl Record {
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

impl From<User> for Record {
    fn from(value: User) -> Self {
        Record {
            id: value.id.into(),
            nickname: value.nickname,
            rating:  value.rating as i16,
            email: value.email,
            password: value.password,
            settings: serde_json::to_value(value.settings).unwrap(),
            created_at: value.created_at.into(),
        }
    }
}

impl From<Row> for User {
    fn from(value: Row) -> Self {
        let rating: i16 = value.get(RecordIden::Rating.to_string().as_str());
        Self {
            id: value.get(RecordIden::Id.to_string().as_str()),
            nickname: value.get(RecordIden::Nickname.to_string().as_str()),
            rating:  rating as u16,
            email: value.get(RecordIden::Email.to_string().as_str()),
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
            .values_panic(Record::from(u).values_skip_id())
            .returning_all()
            .to_owned();

        let sql = q.to_string(PostgresQueryBuilder);

        let conn = self.pg.get().await?;

        Ok(conn.query_one(&sql, &[]).await?.into())
    }

    pub async fn find(&self, o: &UserOptions) -> Result<User> {
        let users = self.find_many(o).await?;

        // @TODO прочекать на что вернет файнд мэни для пустоты
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
            q.and_where(Expr::col(RecordIden::Id).eq(*id as i64));
        }

        if let Some(email) = o.email.as_ref() {
            q.and_where(Expr::col(RecordIden::Email).eq(email.as_str()));
        }

        if let Some(nickname) = o.nickname.as_ref() {
            q.and_where(Expr::col(RecordIden::Nickname).eq(nickname.as_str()));
        }

        let sql = q.to_string(PostgresQueryBuilder);
        let conn = self.pg.get().await?;

        let rows = conn.query(&sql, &[]).await?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }
}