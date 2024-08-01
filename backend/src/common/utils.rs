use uuid::Uuid;

use diesel::result::Error;

use super::errors::UuidParseError;

pub fn parse_uuid(uuid: &str) -> Result<Uuid, Error> {
    match Uuid::parse_str(uuid) {
        Ok(id_) => Ok(id_),
        Err(_) => {
            return Err(diesel::result::Error::DeserializationError(Box::new(
                UuidParseError::new("Failed to parse Uuid"),
            )))
        }
    }
}
