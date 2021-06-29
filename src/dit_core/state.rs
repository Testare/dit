use super::Mode;

pub trait State: Default {
    fn read_header(header: &str) -> Self;
    fn root_hash(&self) -> &str;
    fn mode() -> Mode;
}
