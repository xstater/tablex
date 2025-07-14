pub trait Executor {
    type Output;

    fn execute(&mut self) -> rusqlite::Result<Self::Output>;

    fn sql(&mut self) -> String;
}
