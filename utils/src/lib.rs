pub mod id;


#[cfg(test)]
mod tests {
    use super::*;
    use id;

    #[test]
    fn test_id() {
        let mut  ig =  id::IdGenerator::new(1,1);
        assert!(ig.next_id()!=ig.next_id());
        assert!(ig.new_uuid()!=ig.new_uuid());
        assert!(ig.new_uuid_str()!=ig.new_uuid_str());
        println!("id: {}, uuid: {} uuid_str:{}", ig.next_id(), ig.new_uuid(), ig.new_uuid_str());
    }
}
