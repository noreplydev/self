#[derive(Debug)]
pub struct Str {
    pub value: String,
}
impl Str {
    pub fn new(value: String) -> Str {
        Str { value }
    }
}
