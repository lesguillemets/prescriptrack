use crate::Prescription;

pub trait Parser {
    fn parse(&self, input: &str) -> Prescription;
    fn parse_all(&self, input: &str) -> Vec<Prescription>;
}
