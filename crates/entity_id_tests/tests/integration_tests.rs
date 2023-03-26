use ulid::Ulid;

use entity_id::EntityId;

#[derive(EntityId, Debug)]
#[entity_id(prefix = "user")]
struct UserId(Ulid);

#[test]
fn prefix_associated_const() {
    assert_eq!(UserId::PREFIX, "user")
}

#[test]
fn new_generates_an_id_with_the_given_prefix() {
    let user_id = UserId::new();

    assert!(user_id.to_string().starts_with("user_"));
}

#[test]
fn unprefixed_returns_the_id_without_the_prefix() {
    let user_id = UserId::new();

    assert_eq!(user_id.unprefixed(), user_id.0.to_string().to_lowercase());
}

#[cfg(feature = "uuid")]
#[test]
fn entity_id_from_uuid() {
    use uuid::Uuid;

    let uuid = Uuid::try_from("14a20d59-4d68-4bdf-aac6-8e1af037d183").unwrap();

    let user_id = UserId::from(uuid);

    assert_eq!(user_id.to_string(), "user_0mm86njkb89fftnhme3br3fmc3");
}

#[cfg(feature = "uuid")]
#[test]
fn uuid_from_entity_id() {
    use std::str::FromStr;

    use uuid::Uuid;

    let user_id = UserId::from_str("user_2wdncp35529bet2md0kzxrj0bs").unwrap();

    let uuid = Uuid::from(user_id);

    assert_eq!(
        uuid,
        Uuid::from_str("5c6d5961-94a2-4add-a151-a09ffb890179").unwrap()
    )
}
