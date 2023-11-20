
use sqlx::types::BigDecimal;

pub mod accounts;
pub mod cat_types;
pub mod trx_cats;
pub mod trxs;

pub fn bigdecimal_to_int(value: BigDecimal) -> i64 {
  let (big_int, _) = value.into_bigint_and_exponent();
  let int_value = big_int.to_string().parse::<i64>().unwrap();

  int_value
}