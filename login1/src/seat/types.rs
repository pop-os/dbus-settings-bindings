use serde::{Deserialize, Serialize};
use zbus::{OwnedObjectPath, OwnedVariant, Structure, Type};

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize)]
pub struct SessionPath {
	id: String,
	/// Name of session user
	path: OwnedObjectPath,
}

impl SessionPath {
	pub fn id(&self) -> &str {
		&self.id
	}

	pub fn path(&self) -> &OwnedObjectPath {
		&self.path
	}
}

impl TryFrom<OwnedVariant> for SessionPath {
	type Error = zbus::Error;

	fn try_from(value: OwnedVariant) -> Result<Self, Self::Error> {
		let value = <Structure<'_>>::try_from(value)?;
		Ok(Self {
			id: <String>::try_from(value.fields()[0].try_clone()?)?,
			path: <OwnedObjectPath>::try_from(value.fields()[1].try_clone()?)?,
		})
	}
}
