use serde::{Deserialize, Serialize};

use crate::B64Url;
#[cfg(doc)]
use crate::{Account, Collection, Item};

/// Defines entity sharing between user accounts
///
/// Entities are shared by applying the [`SharedExtension`] extension to them.
/// This extensions MUST only be applied to [`Collection`] and [`Item`].
///
/// Entities that are shared MUST only be included in the exports for accounts that are credential
/// owners or admins of the entity.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SharedExtension {
    /// A list of [`SharingAccessor`] objects that represents users or groups
    /// and their permissions with respect to access on the entity to which the [`SharedExtension`]
    /// is applied.
    pub accessors: Vec<SharingAccessor>,
}

/// A SharingAccessor represents a user or group and their access permissions with respect to an
/// entity.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SharingAccessor {
    /// Indicates the type of accessor for which permissions are defined.
    /// Importers must ignore any SharingAccessor entries when this value is
    /// [`SharingAccessorType::Unknown`].
    #[serde(rename = "type")]
    pub ty: SharingAccessorType,
    /// This member specifies the [`Account`], identified by its [`Account::id`],
    /// that has been given access to the shared entity by the current exporting Account.
    pub account_id: B64Url,
    /// This contains the accessor’s account name.
    /// If [`Self::ty`] has the value [`SharingAccessorType::User`] this SHOULD be set to the
    /// [`Account::username`]. If [`Self::ty`] has the value [`SharingAccessorType::Group`]
    /// this SHOULD be set to the group’s name.
    pub name: String,
    /// The list of permissions that [`Account`] defined by [`Self::account_id`] has with respect
    /// to access on the shared entity. Importers MUST ignore entries with value of
    /// [`SharingAccessorPermission::Unknown`]. Importers MUST ignore any [`SharingAccessors=]
    /// that have an empty permissions list, whether it’s been exported as empty or when it’s
    /// empty as a result of ignoring all unknown entries.
    pub permissions: Vec<SharingAccessorPermission>,
}

/// A SharingAccessorType indicates the type of accessor for which a [`SharingAccessor`] defines
/// access permissions to the respective entity.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum SharingAccessorType {
    /// Indicates the respective [`SharingAccessor`] is describing a specific user’s [`Account`]'s
    /// permissions on the shared entity.
    User,
    /// Indicates the respective [`SharingAccessor`] is describing a group of user’s permissions on
    /// the shared entity.
    Group,
    /// An unknown [`SharingAccessorType`], this is meant for future compatibility.
    #[serde(untagged)]
    Unknown(String),
}

/// The SharingAccessorPermission encodes the level of access the accessing [`Account`] is given to
/// the respective entity.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum SharingAccessorPermission {
    /// Indicates that the respective [`SharingAccessor`] has read permissions on the associated
    /// entity, excluding its secrets. This generally means that the client prevents the user
    /// from revealing the secret (e.g., a password) in its interface. However, the user is
    /// often still allowed to use the secrets in an autofill context.
    Read,
    /// Indicates that the respective [`SharingAccessor`] has read permissions on the associated
    /// entity, including its secrets.
    ReadSecret,
    /// Indicates that the respective [`SharingAccessor`] has update permissions on the associated
    /// entity.
    Update,
    /// Indicates that the respective [`SharingAccessor`] has the permission to create sub-entities
    /// for the associated entity, if applicable.
    Create,
    /// Indicates that the respective [`SharingAccessor`] has the permission to delete any of the
    /// associated entity’s sub-entities, if applicable.
    Delete,
    /// Indicates that the respective [`SharingAccessor`] can share any of the associated entity’s
    /// sub-entities with users or groups, if applicable.
    Share,
    /// Indicates that the respective [`SharingAccessor`] can manage the associated entity,
    /// meaning they can edit the entity’s attributes, share it with others, etc.
    Manage,
    /// An unknown permission, this is meant for future compatibility.
    #[serde(untagged)]
    Unknown(String),
}
