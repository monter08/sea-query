#[macro_export]
macro_rules! bind_params_sqlx_postgres {
    ( $query:expr, $params:expr ) => {{
        let mut query = $query;
        for value in $params.iter() {
            macro_rules! bind {
                ( $v: expr, $ty: ty ) => {
                    match $v {
                        Some(v) => query.bind((*v as $ty)),
                        None => query.bind(None::<$ty>),
                    }
                };
            }
            macro_rules! bind_box {
                ( $v: expr, $ty: ty ) => {
                    match $v {
                        Some(v) => query.bind(v.as_ref()),
                        None => query.bind(None::<$ty>),
                    }
                };
            }
            query = match value {
                Value::Bool(v) => bind!(v, bool),
                Value::TinyInt(v) => bind!(v, i8),
                Value::SmallInt(v) => bind!(v, i16),
                Value::Int(v) => bind!(v, i32),
                Value::BigInt(v) => bind!(v, i64),
                Value::TinyUnsigned(v) => bind!(v, u32),
                Value::SmallUnsigned(v) => bind!(v, u32),
                Value::Unsigned(v) => bind!(v, u32),
                Value::BigUnsigned(v) => bind!(v, i64),
                Value::Float(v) => bind!(v, f32),
                Value::Double(v) => bind!(v, f64),
                Value::String(v) => bind_box!(v, String),
                Value::Bytes(v) => bind_box!(v, Vec<u8>),
                _ => {
                    if value.is_json() {
                        query.bind(value.as_ref_json())
                    } else if value.is_time_date() {
                        query.bind(value.as_ref_time_date())
                    } else if value.is_time_time() {
                        query.bind(value.as_ref_time_time())
                    } else if value.is_time_date_time() {
                        query.bind(value.as_ref_time_date_time())
                    } else if value.is_chrono_date_time_with_time_zone() {
                        query.bind(value.as_ref_chrono_date_time_with_time_zone())
                    } else if value.is_decimal() {
                        query.bind(value.as_ref_decimal())
                    }
                    // else if value.is_big_decimal() {
                    //     query.bind(value.as_ref_big_decimal())
                    // }
                    else if value.is_uuid() {
                        query.bind(value.as_ref_uuid())
                    } else if value.is_array() {
                        if value.is_bool_array() {
                            query.bind(value.as_bool_ref_array())
                        } else if value.is_tiny_int_array() {
                            query.bind(value.as_tiny_int_ref_array())
                        } else if value.is_small_int_array() {
                            query.bind(value.as_small_int_ref_array())
                        } else if value.is_int_array() {
                            query.bind(value.as_int_ref_array())
                        } else if value.is_big_int_array() {
                            query.bind(value.as_big_int_ref_array())
                        } else if value.is_tiny_unsigned_array() {
                            query.bind(value.as_tiny_unsigned_ref_array())
                        } else if value.is_small_unsigned_array() {
                            query.bind(value.as_small_unsigned_ref_array())
                        } else if value.is_unsigned_array() {
                            query.bind(value.as_unsigned_ref_array())
                        } else if value.is_big_unsigned_array() {
                            query.bind(value.as_big_unsigned_ref_array())
                        } else if value.is_float_array() {
                            query.bind(value.as_float_ref_array())
                        } else if value.is_double_array() {
                            query.bind(value.as_double_ref_array())
                        } else if value.is_string_array() {
                            query.bind(value.as_string_ref_array())
                        } else if value.is_json_array() {
                            query.bind(value.as_json_ref_array())
                        } else if value.is_date_array() {
                            query.bind(value.as_date_ref_array())
                        } else if value.is_time_array() {
                            query.bind(value.as_time_ref_array())
                        } else if value.is_date_time_array() {
                            query.bind(value.as_date_time_ref_array())
                        } else if value.is_date_time_with_time_zone_array() {
                            query.bind(value.as_date_time_with_time_zone_ref_array())
                        }
                        else if value.is_decimal_array() {
                            query.bind(value.as_decimal_ref_array())
                        }
                        // else if value.is_big_decimal_array() { // @todo support
                        //     query.bind(value.as_big_decimal_ref_array())
                        // }
                        else if value.is_uuid_array() {
                            query.bind(value.as_uuid_ref_array())
                        } else {
                            unimplemented!();
                        }
                    } else {
                        unimplemented!();
                    }
                }
            };
        }
        query
    }};
}

#[macro_export]
macro_rules! sea_query_driver_postgres {
    () => {
        mod sea_query_driver_postgres {
            use sqlx::{postgres::PgArguments, query::Query, query::QueryAs, Postgres};
            use $crate::{Value, Values};

            type SqlxQuery<'a> = sqlx::query::Query<'a, Postgres, PgArguments>;
            type SqlxQueryAs<'a, T> = sqlx::query::QueryAs<'a, Postgres, T, PgArguments>;

            pub fn bind_query<'a>(query: SqlxQuery<'a>, params: &'a Values) -> SqlxQuery<'a> {
                $crate::bind_params_sqlx_postgres!(query, params.0)
            }

            pub fn bind_query_as<'a, T>(
                query: SqlxQueryAs<'a, T>,
                params: &'a Values,
            ) -> SqlxQueryAs<'a, T> {
                $crate::bind_params_sqlx_postgres!(query, params.0)
            }
        }
    };
}
