use bevy::prelude::Component;

#[derive(Component, Debug, Hash)]
pub struct Coordinates(pub i32, pub i32);
impl PartialEq for Coordinates {
    fn eq(&self, other: &Self) -> bool {
	return self.0 == other.0 && self.1 == other.1
    }
}
impl Eq for Coordinates {}
