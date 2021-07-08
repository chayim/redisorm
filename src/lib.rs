#[macro_use]

extern crate redis_module;

use redis_module::{Context, RedisError, RedisResult, RedisString, RedisValue};

fn orm_sql(_: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }
    let mut response = String::new();
     for i in 1..args.len() {
         response.push_str(&args[i].into_string_lossy());
         if i != args.len()-1 {
             response.push_str(" ");
        }
     }
    return Ok(response.into());
}

//////////////////////////////////////////////////////

redis_module! {
    name: "redisorm",
    version: 1,
    data_types: [],
    commands: [
        ["orm.sql", orm_sql, "write", 0, 0, 0],
    ],
}