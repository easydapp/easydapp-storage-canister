use std::cell::RefCell;

use jelly_model::{
    store::{
        api::{
            anchor::{ApiDataHash, ApiDataParsedId},
            ApiData,
        },
        code::{
            anchor::{CodeDataHash, CodeDataParsedId},
            CodeData,
        },
        combined::{
            anchor::{CombinedHash, CombinedParsedId},
            Combined,
        },
        dapp::{
            access::{DappAccess, DappAccessView, DappVerified},
            anchor::DappParsedId,
            Dapp, DappView,
        },
        publisher::{
            anchor::{PublisherId, PublisherParsedId},
            Publisher,
        },
    },
    types::TimestampMills,
};
use serde::{Deserialize, Serialize};

use crate::types::*;

#[derive(Serialize, Deserialize)]
pub struct State {
    /// administrator
    #[serde(skip, default = "init_admin_data")]
    admin: StableCell<AdminUsers>,

    /// Publisher
    #[serde(skip, default = "init_publisher_data")]
    publisher: StableBTreeMap<PublisherId, Publisher>,

    #[serde(skip, default = "init_code_data")]
    code: StableBTreeMap<CodeDataHash, CodeData>,

    #[serde(skip, default = "init_apis_data")]
    apis: StableBTreeMap<ApiDataHash, ApiData>,

    #[serde(skip, default = "init_combined_data")]
    combined: StableBTreeMap<CombinedHash, Combined>,
    #[serde(skip, default = "init_combined_called_data")]
    combined_called: StableBTreeMap<CombinedHash, u64>,

    #[serde(skip, default = "init_dapp_data")]
    dapp: StableBTreeMap<WrappedDappId, Dapp>,
    #[serde(skip, default = "init_dapp_accesses_data")]
    dapp_accesses: StableBTreeMap<WrappedDappId, DappAccess>,
    #[serde(skip, default = "init_dapp_accessed_data")]
    dapp_accessed: StableBTreeMap<WrappedDappId, u64>, // Statistical number
    #[serde(skip, default = "init_dapp_called_data")]
    dapp_called: StableBTreeMap<WrappedDappId, u64>,
    #[serde(skip, default = "init_dapp_collected_data")]
    dapp_collected: StableBTreeMap<WrappedDappId, u64>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            admin: init_admin_data(),

            publisher: init_publisher_data(),

            code: init_code_data(),

            apis: init_apis_data(),

            combined: init_combined_data(),
            combined_called: init_combined_called_data(),

            dapp: init_dapp_data(),
            dapp_accesses: init_dapp_accesses_data(),
            dapp_accessed: init_dapp_accessed_data(),
            dapp_called: init_dapp_called_data(),
            dapp_collected: init_dapp_collected_data(),
        }
    }
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static STATE: RefCell<State> = RefCell::default();
}

const MEMORY_ID_ADMIN: MemoryId = MemoryId::new(0); // Administrator data

const MEMORY_ID_PUBLISHER: MemoryId = MemoryId::new(10); // Publisher metadata

const MEMORY_ID_CODE: MemoryId = MemoryId::new(20); // Code data

const MEMORY_ID_APIS: MemoryId = MemoryId::new(30); // Api data

const MEMORY_ID_COMBINED: MemoryId = MemoryId::new(40); // Content data
const MEMORY_ID_COMBINED_CALLED: MemoryId = MemoryId::new(41); // combined data

const MEMORY_ID_DAPP: MemoryId = MemoryId::new(50); // dapp data
const MEMORY_ID_DAPP_ACCESSES: MemoryId = MemoryId::new(51); // dapp data
const MEMORY_ID_DAPP_ACCESSED: MemoryId = MemoryId::new(52); // dapp data
const MEMORY_ID_DAPP_CALLED: MemoryId = MemoryId::new(53); // dapp data
const MEMORY_ID_DAPP_COLLECTED: MemoryId = MemoryId::new(54); // dapp data

fn get_virtual_memory(memory_id: MemoryId) -> VirtualMemory {
    MEMORY_MANAGER.with(|memory_manager| memory_manager.borrow().get(memory_id))
}

// =============== admin ===============

fn init_admin_data() -> StableCell<AdminUsers> {
    #[allow(clippy::expect_used)] // ? SAFETY
    StableCell::init(get_virtual_memory(MEMORY_ID_ADMIN), Default::default()).expect("failed to initialize")
}

impl Storable for AdminUsers {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut bytes = vec![];
        #[allow(clippy::unwrap_used)] // ? SAFETY
        ciborium::ser::into_writer(self, &mut bytes).unwrap();
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        #[allow(clippy::expect_used)] // ? SAFETY
        ciborium::de::from_reader(&bytes[..]).expect("deserialization must succeed.")
    }

    const BOUND: Bound = Bound::Unbounded;
}

// =============== publisher ===============

fn init_publisher_data() -> StableBTreeMap<PublisherId, Publisher> {
    StableBTreeMap::init(get_virtual_memory(MEMORY_ID_PUBLISHER))
}

// =============== code ===============

fn init_code_data() -> StableBTreeMap<CodeDataHash, CodeData> {
    StableBTreeMap::init(get_virtual_memory(MEMORY_ID_CODE))
}
// =============== apis ===============

fn init_apis_data() -> StableBTreeMap<ApiDataHash, ApiData> {
    StableBTreeMap::init(get_virtual_memory(MEMORY_ID_APIS))
}

// =============== combined ===============

fn init_combined_data() -> StableBTreeMap<CombinedHash, Combined> {
    StableBTreeMap::init(get_virtual_memory(MEMORY_ID_COMBINED))
}
fn init_combined_called_data() -> StableBTreeMap<CombinedHash, u64> {
    StableBTreeMap::init(get_virtual_memory(MEMORY_ID_COMBINED_CALLED))
}

// =============== dapp ===============

fn init_dapp_data() -> StableBTreeMap<WrappedDappId, Dapp> {
    StableBTreeMap::init(get_virtual_memory(MEMORY_ID_DAPP))
}
fn init_dapp_accesses_data() -> StableBTreeMap<WrappedDappId, DappAccess> {
    StableBTreeMap::init(get_virtual_memory(MEMORY_ID_DAPP_ACCESSES))
}
fn init_dapp_accessed_data() -> StableBTreeMap<WrappedDappId, u64> {
    StableBTreeMap::init(get_virtual_memory(MEMORY_ID_DAPP_ACCESSED))
}
fn init_dapp_called_data() -> StableBTreeMap<WrappedDappId, u64> {
    StableBTreeMap::init(get_virtual_memory(MEMORY_ID_DAPP_CALLED))
}
fn init_dapp_collected_data() -> StableBTreeMap<WrappedDappId, u64> {
    StableBTreeMap::init(get_virtual_memory(MEMORY_ID_DAPP_COLLECTED))
}

#[allow(unused)]
pub fn with_state<F, R>(callback: F) -> R
where
    F: FnOnce(&State) -> R,
{
    STATE.with(|_state| {
        let state = _state.borrow();
        callback(&state)
    })
}

#[allow(unused)]
pub fn with_mut_state<F, R>(callback: F) -> R
where
    F: FnOnce(&mut State) -> R,
{
    STATE.with(|_state| {
        let mut state = _state.borrow_mut();
        callback(&mut state)
    })
}

pub fn must_be_admin() -> Result<(), String> {
    let caller = ic_cdk::caller();
    with_state(|s| {
        if s.is_admin(&caller) {
            return Ok(());
        }
        Err("Permission is required".into())
    })
}

/// Get the current time
fn now() -> TimestampMills {
    let now = ic_cdk::api::time() as i64;
    now.into()
}

impl State {
    // ================== admin ==================
    fn is_admin(&self, caller: &Principal) -> bool {
        self.admin.get().is_admin(caller)
    }
    pub fn admin_add(&mut self, user: Principal) {
        let mut item = self.admin.get().to_owned();
        item.users.insert(user);
        #[allow(clippy::unwrap_used)] // ? SAFETY
        self.admin.set(item).unwrap();
    }
    pub fn admin_remove(&mut self, user: &Principal) {
        let mut item = self.admin.get().to_owned();
        item.users.remove(user);
        #[allow(clippy::unwrap_used)] // ? SAFETY
        self.admin.set(item).unwrap();
    }
    pub fn admin_query(&self) -> Vec<Principal> {
        self.admin.get().users.iter().copied().collect()
    }

    // ================== publisher ==================
    // ! Administrator insert
    pub fn publisher_update(&mut self, publisher: Publisher) {
        #[allow(clippy::unwrap_used)] // ? SAFETY
        let id: PublisherParsedId = publisher.anchor.as_ref().as_str().try_into().unwrap();
        #[allow(clippy::unwrap_used)] // ? SAFETY
        id.check_canister_id(&ic_cdk::id()).unwrap();
        let key = &id.id; // key

        self.publisher.insert(key.to_owned(), publisher);
    }
    pub fn publisher_query(&self, id: PublisherParsedId) -> Option<Publisher> {
        id.check_canister_id(&ic_cdk::id()).ok()?;
        let key = &id.id; // key

        self.publisher.get(key)
    }

    // ================== code ==================
    // ! Administrator insert
    pub fn code_update(&mut self, code: CodeData) {
        #[allow(clippy::unwrap_used)] // ? SAFETY
        let id: CodeDataParsedId = code.anchor.as_ref().as_str().try_into().unwrap();
        #[allow(clippy::unwrap_used)] // ? SAFETY
        id.check_canister_id(&ic_cdk::id()).unwrap();
        let key = &id.hash; // key

        #[allow(clippy::panic)] // ? SAFETY
        if let Some(c) = self.code.get(key) {
            if c.code != code.code || c.js.trim() != code.js.trim() {
                panic!(
                    "code already exists: {:?} {:?} vs {:?} {:?}",
                    c.code, c.js, code.code, code.js
                );
            }
            return;
        }

        self.code.insert(key.to_owned(), code);
    }
    pub fn code_query(&self, id: CodeDataParsedId) -> Option<CodeData> {
        id.check_canister_id(&ic_cdk::id()).ok()?;
        let key = &id.hash; // key

        self.code.get(key)
    }

    // ================== apis ==================

    // ! Administrator insert
    pub fn apis_update(&mut self, api: ApiData) {
        #[allow(clippy::unwrap_used)] // ? SAFETY
        let id: ApiDataParsedId = api.anchor.as_ref().as_str().try_into().unwrap();
        #[allow(clippy::unwrap_used)] // ? SAFETY
        id.check_canister_id(&ic_cdk::id()).unwrap();
        let key = &id.hash; // key

        #[allow(clippy::panic)] // ? SAFETY
        if let Some(a) = self.apis.get(key) {
            if a.content != api.content {
                panic!("api already exists");
            }
            return;
        }

        self.apis.insert(key.to_owned(), api);
    }
    pub fn apis_query(&self, id: ApiDataParsedId) -> Option<ApiData> {
        id.check_canister_id(&ic_cdk::id()).ok()?;
        let key = &id.hash; // key

        self.apis.get(key)
    }

    // ================== combined ==================

    fn inner_combined_increment_called(&mut self, key: CombinedHash) -> Result<(), String> {
        if let Some(called) = self.combined_called.get(&key) {
            self.combined_called.insert(key, called + 1);
        }
        Ok(())
    }
    fn inner_combined_query(&self, key: CombinedHash) -> Option<Combined> {
        if let Some(mut combined) = self.combined.get(&key) {
            combined.called = self.combined_called.get(&key).unwrap_or_default();
            return Some(combined);
        }
        None
    }

    // ! Administrator insert
    pub fn combined_update(&mut self, combined: Combined) {
        #[allow(clippy::unwrap_used)] // ? SAFETY
        let id: CombinedParsedId = combined.anchor.as_ref().as_str().try_into().unwrap();
        #[allow(clippy::unwrap_used)] // ? SAFETY
        id.check_canister_id(&ic_cdk::id()).unwrap();
        let key = &id.hash; // key

        // The same content is not allowed to be inserted
        #[allow(clippy::panic)] // ? SAFETY
        if let Some(o) = self.combined.get(key) {
            if o.components != combined.components {
                panic!("combined already exists");
            }
            return;
        }

        self.combined_called.insert(key.to_owned(), combined.called);
        self.combined.insert(key.to_owned(), combined);
    }
    pub fn combined_increment_called(&mut self, id: CombinedParsedId) -> Result<(), String> {
        id.check_canister_id(&ic_cdk::id())?;
        let key = &id.hash; // key
        self.inner_combined_increment_called(key.to_owned())
    }
    // ! Administrator call
    pub fn combined_query(&self, id: CombinedParsedId) -> Option<Combined> {
        id.check_canister_id(&ic_cdk::id()).ok()?;
        let key = &id.hash; // key
        self.inner_combined_query(key.to_owned())
    }

    // ================== dapp ==================

    // The current jar can only check these two items
    fn inner_dapp_access_by_timestamp_and_token(
        &self,
        key: &WrappedDappId,
        verified: Option<DappVerified>,
    ) -> Result<(), String> {
        if let Some(access) = self.dapp_accesses.get(key) {
            if !access.access_by_timestamp_and_token(now(), verified.as_ref()) {
                return Err(format!("access is deny: {}", key.0.as_ref()));
            }
        }
        Ok(())
    }
    fn inner_dapp_increment_accessed(&mut self, key: WrappedDappId) -> Result<(), String> {
        if let Some(accessed) = self.dapp_accessed.get(&key) {
            self.dapp_accessed.insert(key.clone(), accessed + 1);
        }
        Ok(())
    }
    fn inner_dapp_increment_called(&mut self, key: WrappedDappId) -> Result<(), String> {
        if let Some(called) = self.dapp_called.get(&key) {
            self.dapp_called.insert(key.clone(), called + 1);
        }
        Ok(())
    }
    #[allow(unused)]
    fn inner_dapp_query_with_increment_accessed(&mut self, key: WrappedDappId) -> Result<DappView, String> {
        if let Some(mut dapp) = self.dapp.get(&key) {
            if dapp.frozen.is_some() {
                return Err(format!("dapp is frozen: {}", key.0.as_ref()));
            }
            dapp.access = self
                .dapp_accesses
                .get(&key)
                .ok_or_else(|| "dapp access is missing".to_string())?;
            dapp.accessed = self.dapp_accessed.get(&key).unwrap_or_default();
            dapp.called = self.dapp_called.get(&key).unwrap_or_default();
            dapp.collected = self.dapp_collected.get(&key).unwrap_or_default();
            self.inner_dapp_increment_accessed(key)?;
            return Ok(dapp.into());
        }
        Err(format!("dapp is missing: {}", key.0.as_ref()))
    }
    fn inner_dapp_query(&self, key: WrappedDappId, admin: bool) -> Result<Dapp, String> {
        if let Some(mut dapp) = self.dapp.get(&key) {
            if !admin && dapp.frozen.is_some() {
                return Err(format!("Frozen: {}", dapp.reason));
            }
            dapp.access = self
                .dapp_accesses
                .get(&key)
                .ok_or_else(|| "dapp access is missing".to_string())?;
            dapp.accessed = self.dapp_accessed.get(&key).unwrap_or_default();
            dapp.called = self.dapp_called.get(&key).unwrap_or_default();
            dapp.collected = self.dapp_collected.get(&key).unwrap_or_default();
            return Ok(dapp);
        }
        Err(format!("dapp is missing: {}", key.0.as_ref()))
    }

    // ! Administrator insert
    pub fn dapp_update(&mut self, dapp: Dapp) {
        #[allow(clippy::unwrap_used)] // ? SAFETY
        let id: DappParsedId = dapp.id.as_ref().as_str().try_into().unwrap();
        #[allow(clippy::unwrap_used)] // ? SAFETY
        id.check_canister_id(&ic_cdk::id()).unwrap();
        let id: WrappedDappId = id.into(); // key

        self.dapp_accesses.insert(id.clone(), dapp.access.to_owned());
        self.dapp_accessed.insert(id.clone(), dapp.accessed);
        self.dapp_collected.insert(id.clone(), dapp.collected);
        self.dapp_called.insert(id.clone(), dapp.called);
        self.dapp.insert(id, dapp);
    }
    // ! Administrator modification
    pub fn dapp_increment_called_by_admin(&mut self, id: DappParsedId) -> Result<(), String> {
        id.check_canister_id(&ic_cdk::id())?;
        let id: WrappedDappId = id.into(); // key

        self.inner_dapp_increment_called(id)
    }
    // ! Administrator modification
    pub fn dapp_update_collected(&mut self, id: DappParsedId, collected: u64) -> Result<(), String> {
        id.check_canister_id(&ic_cdk::id())?;
        let id: WrappedDappId = id.into(); // key

        if self.dapp_collected.get(&id).is_some() {
            self.dapp_collected.insert(id, collected);
        }

        Ok(())
    }
    // ! Administrator call
    pub fn dapp_query_by_admin(&self, id: DappParsedId) -> Result<Dapp, String> {
        id.check_canister_id(&ic_cdk::id())?;
        let id: WrappedDappId = id.into(); // key
        self.inner_dapp_query(id, true) // Do not increase accessed
    }

    /// Ordinary user calls, query the permissions required
    pub fn dapp_query_access(&self, id: DappParsedId) -> Result<DappAccessView, String> {
        id.check_canister_id(&ic_cdk::id())?;
        let id: WrappedDappId = id.into(); // key

        let access = self
            .dapp_accesses
            .get(&id)
            .ok_or_else(|| "dapp access is missing".to_string())?;
        Ok(access.into())
    }
    /// Ordinary users call, pay attention to only the permissions verification of Duration and Token
    pub fn dapp_increment_called_by_token(
        &mut self,
        id: DappParsedId,
        verified: Option<DappVerified>,
    ) -> Result<(), String> {
        id.check_canister_id(&ic_cdk::id())?;
        let id: WrappedDappId = id.into(); // key

        // ! Check the access permissions
        self.inner_dapp_access_by_timestamp_and_token(&id, verified)?;

        self.inner_dapp_increment_called(id)
    }
    /// Ordinary users call, pay attention to only the permissions verification of Duration and Token
    pub fn dapp_query_by_token(&self, id: DappParsedId, verified: Option<DappVerified>) -> Result<DappView, String> {
        id.check_canister_id(&ic_cdk::id())?;
        let id: WrappedDappId = id.into(); // key

        // ! Check the access permissions
        self.inner_dapp_access_by_timestamp_and_token(&id, verified)?;

        self.inner_dapp_query(id, false).map(|dapp| dapp.into()) // Do not increase accessed
    }
}

impl AdminUsers {
    fn is_admin(&self, caller: &Principal) -> bool {
        self.users.contains(caller)
    }
}
