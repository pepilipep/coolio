pub mod psql;

pub trait Storage {
    fn create_playlist(&self) -> ();
}
