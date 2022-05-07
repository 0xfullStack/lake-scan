
use std::ops::Deref;


use actix::{Addr, SyncArbiter, Handler, Message, Actor, SyncContext};
use actix::fut::err;
use actix_web::{error, Error};

use crate::db::postgres::*;

const DB_MAX_CONNECTION: usize = 3;

#[derive(Clone)]
pub struct State {
    pub inner: Addr<Database>,
}

impl State {
    pub fn init() -> State {
        let pool = init_pool().expect("Failed to create pool");
        let addr = SyncArbiter::start(
            DB_MAX_CONNECTION,
            move || Database { pool: pool.clone() }
        );
        let state = State {
            inner: addr.clone()
        };
        state
    }
    pub fn get(&self) -> &Addr<Database> {
        &self.inner
    }
}

