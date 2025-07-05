use anyhow::Ok;
use diesel::{query_dsl::methods::SelectDsl, r2d2::{ConnectionManager, Pool}, MysqlConnection, OptionalExtension, RunQueryDsl, SelectableHelper};
use diesel::query_dsl::methods::FilterDsl;
use diesel::ExpressionMethods;
use uuid::Uuid;
use crate::{db::models::{NewZig, Zig}, zig_error::ZigAnyResult};

use crate::db::schema::zigs::dsl::*;

pub struct Dao {
    pub pool: Pool<ConnectionManager<MysqlConnection>>
}

impl Dao {

    pub fn new(_pool: &Pool<ConnectionManager<MysqlConnection>>) -> Self {
        Dao {
            pool: _pool.clone()
        }
    }

    pub fn create_zig(&self, user: &str) -> ZigAnyResult<Zig> {
        let mut conn = self.pool.get()?;

        let new_zig = NewZig {
            id: &Uuid::new_v4().to_string(),
            user_name: user,
            button_counter: Some(0),
            ash_counter: Some(0),
        };

        diesel::insert_into(zigs)
            .values(&new_zig)
            .execute(&mut conn)?;

        let created = zigs
            .filter(id.eq(new_zig.id))
            .first::<Zig>(&mut conn)?;

        Ok(created)
    }

    pub fn find_zig_by_id(&self, zig_id_: &str) -> ZigAnyResult<Option<Zig>> {
        let mut conn = self.pool.get()?;

        let zig = zigs
            .filter(id.eq(zig_id_))
            .select(Zig::as_select())
            .first::<Zig>(&mut conn)
            .optional()?;

        Ok(zig)
    }

    pub fn increase_button_counter(&self, zig_id_: &str) -> ZigAnyResult<Option<Zig>> {
        let mut conn = self.pool.get()?;

        diesel::update(zigs.filter(id.eq(zig_id_)))
            .set(button_counter.eq(button_counter + 1))
            .execute(&mut conn)?;

        let updated = zigs
            .filter(id.eq(zig_id_))
            .select(Zig::as_select())
            .first::<Zig>(&mut conn)
            .optional()?;

        Ok(updated)
    }

    pub fn increase_ash_counter(&self, zig_id_: &str) -> ZigAnyResult<Option<Zig>> {
        let mut conn = self.pool.get()?;

        diesel::update(zigs.filter(id.eq(zig_id_)))
            .set(ash_counter.eq(ash_counter + 1))
            .execute(&mut conn)?;

        let updated = zigs
            .filter(id.eq(zig_id_))
            .select(Zig::as_select())
            .first::<Zig>(&mut conn)
            .optional()?;

        Ok(updated)
    }
}