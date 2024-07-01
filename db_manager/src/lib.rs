pub mod chat_mapper;
mod connector;
pub mod contact_mapper;
pub mod members_mapper;
pub mod message_mapper;
pub mod user_mapper;

// use data_models;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
