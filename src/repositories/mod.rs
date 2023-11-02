
pub mod accounts;
pub mod trx_cats;

pub trait Executor: Send + Sync {
  type Executor<'this>: Send + Sync + sqlx::MySqlExecutor<'this>;

  // From https://users.rust-lang.org/t/why-does-this-impl-executor-does-not-live-long-enough/94241
  fn _disable_lint(e: Self::Executor<'_>) -> Self::Executor<'_>;

  fn as_executor(&mut self) -> Self::Executor<'_>;
}

impl Executor for sqlx::MySqlPool {
  type Executor<'this> = &'this Self;

  fn _disable_lint(e: Self::Executor<'_>) -> Self::Executor<'_> {
      e
  }

  fn as_executor(&mut self) -> Self::Executor<'_> {
      self
  }
}

impl Executor for sqlx::Transaction<'static, sqlx::MySql> {
  type Executor<'this> = &'this mut sqlx::MySqlConnection;

  fn _disable_lint(e: Self::Executor<'_>) -> Self::Executor<'_> {
      e
  }

  fn as_executor(&mut self) -> Self::Executor<'_> {
      self
  }
}