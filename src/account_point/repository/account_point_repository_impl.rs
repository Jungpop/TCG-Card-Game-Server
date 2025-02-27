use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use diesel::query_dsl::methods::{FilterDsl};
use diesel::{Connection, MysqlConnection, QueryDsl, ExpressionMethods, RunQueryDsl};

use crate::account_point::entity::account_point::AccountPoint;
use crate::account_point::entity::account_point::account_points::dsl::{account_id, account_points};
use crate::account_point::entity::account_point::account_points::columns;
use crate::account_point::repository::account_point_repository::AccountPointRepository;
use crate::common::env::env_detector::EnvDetector;
use crate::mysql_config::mysql_connection::MysqlDatabaseConnection;

pub struct AccountPointRepositoryImpl {
    mysql_database_connection: Arc<AsyncMutex<MysqlDatabaseConnection>>,
}

impl AccountPointRepositoryImpl {
    pub fn new(mysql_connection: Arc<AsyncMutex<MysqlDatabaseConnection>>) -> Self {
        AccountPointRepositoryImpl {
            mysql_database_connection: mysql_connection
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<AccountPointRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<AccountPointRepositoryImpl>> =
                Arc::new(AsyncMutex::new(AccountPointRepositoryImpl::new(
                    MysqlDatabaseConnection::get_instance()
            )));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl AccountPointRepository for AccountPointRepositoryImpl {

    async fn set_account_point(&self, account_user_id: i32, golds: i32) -> AccountPoint {
        Some(AccountPoint::new(account_user_id, golds)).unwrap().expect("REASON")
    }

    async fn save_account_points(&self, account_point: AccountPoint) -> Result<(), diesel::result::Error> {
        use crate::account_point::entity::account_point::account_points::dsl::*;

        println!("AccountRepositoryImpl: save()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        match diesel::insert_into(account_points)
            .values(&account_point)
            .execute(&mut connection)
        {
            Ok(_) => {
                println!("Account saved successfully.");
                Ok(())
            }
            Err(e) => {
                eprintln!("Error saving account: {:?}", e);
                Err(e)
            }
        }
    }

    async fn find_by_account_id(&self, account_user_id: i32) -> Result<Option<AccountPoint>, diesel::result::Error> {
        use crate::account_point::entity::account_point::account_points::dsl::*;
        use diesel::query_dsl::filter_dsl::FilterDsl;
        use diesel::prelude::*;

        println!("AccountPointRepositoryImpl: find_by_account_id()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let select_clause = account_points.select((columns::account_id, columns::gold));
        let where_clause = FilterDsl::filter(account_points, columns::account_id.eq(account_id));
        let found_accounts = where_clause
            .select((columns::account_id, columns::gold))
            .load::<AccountPoint>(&mut connection)?;

        let found_account = found_accounts
            .into_iter()
            .find(|account| account.account_id == account_user_id);

        Ok(Option::from(found_account))
    }

    async fn update_gold(&self, account_point: AccountPoint, golds: i32) -> Result<usize, diesel::result::Error> {
        println!("AccountPointRepositoryImpl: update_gold()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        match diesel::update(FilterDsl::filter(account_points, columns::account_id.eq(account_point.account_id)))
            .set((
                columns::gold.eq(golds),
            ))
            .execute(&mut connection)
        {
            Ok(num) => {
                println!("Gold points updated successfully.");
                Ok(num)
            }
            Err(e) => {
                eprintln!("Error updating Gold points: {:?}", e);
                Err(e)
            }
        }
    }

    async fn delete_account_points(&self, account_user_id: i32) -> Result<(), diesel::result::Error> {
        use crate::account_point::entity::account_point::account_points::dsl::*;

        println!("AccountPointRepositoryImpl: delete()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        match diesel::delete(FilterDsl::filter(account_points, columns::account_id.eq(account_user_id)))

            .execute(&mut connection)
        {
            Ok(_) => {
                println!("Account_point deleted successfully.");
                Ok(())
            }
            Err(e) => {
                eprintln!("Error deleting account_point: {:?}", e);
                Err(e)
            }
        }
    }
}