use mysql::*;
use mysql::prelude::*;
use crate::Config;

pub struct DB {
    connection: PooledConn,
}

impl DB {

    pub(crate) fn connect(config: &Config) -> std::result::Result<Self, Error> {

        let mut builder = OptsBuilder::new();
        builder = builder.ip_or_hostname(Some(&config.db_host))
            .db_name(Some(&config.db_name))
            .pass(Some(&config.db_password))
            .user(Some(&config.db_user))
            .tcp_port(config.db_port);

        let pool = Pool::new(builder)?;

        Ok(DB {connection: pool.get_conn()?})
    }

    //inserts
    // new user connected
    pub fn user_new(&mut self) {}

    // existing user joined for first time
    pub fn user_join(&mut self) {}

    // user left, insert time spent into Db
    pub fn user_leave(&mut self) {}

    //exerts
    //total time spent for user in time period
    pub fn get_total_time(&mut self, start: i32, end: i32, user: i32) {}

    // total number of users
    pub fn get_total_users(&mut self) {
        let count: i32 = self.connection.query_first("SELECT COUNT(*) FROM TSUser").unwrap().unwrap();

        println!("Number of total users: {:?}", count);
    }
}