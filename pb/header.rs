pub trait PbHeader: 'static {
    fn get_header(&self, key: &str) -> Option<&String>;
}
