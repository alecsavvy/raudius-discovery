use ethcontract::Address;
use serde::{Deserialize, Serialize};

/// Typesafe implementation of the ManageEntity contract event
#[derive(Debug, Clone)]
pub enum Event {
    CreateDeveloperApp {
        base: ManageEntityBase,
        metadata: DeveloperAppMetadata,
    },
    CreateGrant,
    CreateNotification,
    CreatePlaylist {
        base: ManageEntityBase,
        metadata: PlaylistMetadata,
    },
    CreateTrack {
        base: ManageEntityBase,
        metadata: TrackMetadata,
    },
    CreateUser {
        base: ManageEntityBase,
        metadata: UserMetadata,
    },
    DeleteDeveloperApp {
        base: ManageEntityBase,
        metadata: DeveloperAppMetadata,
    },
    DeleteGrant,
    DeletePlaylist {
        base: ManageEntityBase,
        metadata: PlaylistMetadata,
    },
    DeleteTrack {
        base: ManageEntityBase,
        metadata: TrackMetadata,
    },
    FollowUser {
        base: ManageEntityBase,
        metadata: UserMetadata,
    },
    RepostPlaylist {
        base: ManageEntityBase,
        metadata: PlaylistMetadata,
    },
    RepostTrack {
        base: ManageEntityBase,
        metadata: TrackMetadata,
    },
    SavePlaylist {
        base: ManageEntityBase,
        metadata: PlaylistMetadata,
    },
    SaveTrack {
        base: ManageEntityBase,
        metadata: TrackMetadata,
    },
    SubscribeUser {
        base: ManageEntityBase,
        metadata: UserMetadata,
    },
    UnfollowUser {
        base: ManageEntityBase,
        metadata: UserMetadata,
    },
    UnrepostPlaylist {
        base: ManageEntityBase,
        metadata: PlaylistMetadata,
    },
    UnrepostTrack {
        base: ManageEntityBase,
        metadata: TrackMetadata,
    },
    UnsavePlaylist {
        base: ManageEntityBase,
        metadata: PlaylistMetadata,
    },
    UnsaveTrack {
        base: ManageEntityBase,
        metadata: TrackMetadata,
    },
    UnsubscribeUser,
    UpdatePlaylist {
        base: ManageEntityBase,
        metadata: PlaylistMetadata,
    },
    UpdateTrack {
        base: ManageEntityBase,
        metadata: TrackMetadata,
    },
    UpdateUser {
        base: ManageEntityBase,
        metadata: UserMetadata,
    },
    UpdateUserReplicaSet,
    VerifyUser,
    ViewNotification,
    ViewPlaylistNotification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManageEntityBase {
    pub user_id: i64,
    pub signer: Address,
    pub entity_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackMetadata {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistMetadata {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMetadata {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperAppMetadata {}
