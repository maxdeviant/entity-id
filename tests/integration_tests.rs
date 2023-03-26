use entity_id::{self, EntityId};
use ulid::Ulid;

#[derive(EntityId, Debug)]
#[entity_id(prefix = "user")]
struct UserId(Ulid);

#[test]
fn new_generates_an_id_with_the_given_prefix() {
    let user_id = UserId::new();

    assert!(user_id.to_string().starts_with("user_"));
}
