use jelly_model::store::dapp::anchor::DappId;
use jelly_model::store::dapp::anchor::DappParsedId;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;

pub use std::borrow::Cow;

pub use candid::Principal;
pub use ic_stable_structures::memory_manager::MemoryId;
pub use ic_stable_structures::memory_manager::MemoryManager;
pub use ic_stable_structures::storable::Bound;
pub use ic_stable_structures::DefaultMemoryImpl;
pub use ic_stable_structures::Storable;

pub type VirtualMemory = ic_stable_structures::memory_manager::VirtualMemory<ic_stable_structures::DefaultMemoryImpl>;
pub type StableCell<T> = ic_stable_structures::Cell<T, VirtualMemory>;
pub type StableBTreeMap<K, V> = ic_stable_structures::BTreeMap<K, V, VirtualMemory>;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AdminUsers {
    pub users: HashSet<Principal>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrappedDappId(pub DappId, Option<u32>);

impl From<DappParsedId> for WrappedDappId {
    fn from(id: DappParsedId) -> Self {
        Self(id.id, id.nonce)
    }
}

impl Storable for WrappedDappId {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = [0_u8; 12];
        bytes[..8].copy_from_slice(&self.0.to_bytes());
        if let Some(published) = self.1 {
            bytes[8..].copy_from_slice(&published.to_be_bytes());
        }
        Cow::Owned(bytes.to_vec())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let dapp_id = DappId::from_bytes(Cow::Borrowed(&bytes[..8]));
        let mut published_bytes = [0_u8; 4];
        published_bytes.copy_from_slice(&bytes[8..]);
        let published = u32::from_be_bytes(published_bytes);
        Self(dapp_id, (0 < published).then_some(published))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 12,
        is_fixed_size: true,
    };
}
