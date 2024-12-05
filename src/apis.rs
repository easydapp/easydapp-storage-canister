use candid::Principal;
use jelly_model::store::{
    api::{anchor::ApiDataParsedId, ApiData},
    code::{anchor::CodeDataParsedId, CodeData},
    combined::{anchor::CombinedParsedId, Combined},
    dapp::{access::DappVerified, anchor::DappParsedId, Dapp},
    publisher::{anchor::PublisherParsedId, Publisher},
};

use crate::stable::*;

// ================== init ==================

#[ic_cdk::init]
fn initial() {
    let deployer = ic_cdk::caller();
    with_mut_state(|s| s.admin_add(deployer))
}

// ================== admin ==================

#[ic_cdk::update(guard = "must_be_admin")]
fn admin_add(user: Principal) {
    with_mut_state(|s| s.admin_add(user))
}
#[ic_cdk::update(guard = "must_be_admin")]
fn admin_remove(user: Principal) {
    with_mut_state(|s| s.admin_remove(&user))
}
#[ic_cdk::query(guard = "must_be_admin")]
fn admin_query() -> Vec<Principal> {
    with_state(|s| s.admin_query())
}

// ================== user ==================

#[ic_cdk::update(guard = "must_be_admin")]
fn publisher_update(publisher_json: String) {
    #[allow(clippy::unwrap_used)] // ? SAFETY
    let publisher: Publisher = serde_json::from_str(&publisher_json).unwrap();
    with_mut_state(|s| s.publisher_update(publisher))
}
#[ic_cdk::query]
fn publisher_query(anchor: String) -> Option<String> {
    let id: PublisherParsedId = anchor.as_str().try_into().ok()?;
    with_state(|s| s.publisher_query(id)).and_then(|publisher| serde_json::to_string(&publisher).ok())
}

// ================== code ==================

#[ic_cdk::update(guard = "must_be_admin")]
fn code_update(code_json: String) {
    #[allow(clippy::unwrap_used)] // ? SAFETY
    let code: CodeData = serde_json::from_str(&code_json).unwrap();
    with_mut_state(|s| s.code_update(code))
}
#[ic_cdk::query]
fn code_query(anchor: String) -> Option<String> {
    let id: CodeDataParsedId = anchor.as_str().try_into().ok()?;
    with_state(|s| s.code_query(id)).and_then(|code| serde_json::to_string(&code).ok())
}

// ================== apis ==================

#[ic_cdk::update(guard = "must_be_admin")]
fn api_update(api_json: String) {
    #[allow(clippy::unwrap_used)] // ? SAFETY
    let api: ApiData = serde_json::from_str(&api_json).unwrap();
    with_mut_state(|s| s.apis_update(api))
}
#[ic_cdk::query]
fn api_query(anchor: String) -> Option<String> {
    let id: ApiDataParsedId = anchor.as_str().try_into().ok()?;
    with_state(|s| s.apis_query(id)).and_then(|api| serde_json::to_string(&api).ok())
}

// ================== combined ==================

#[ic_cdk::update(guard = "must_be_admin")]
fn combined_update(combined_json: String) {
    #[allow(clippy::unwrap_used)] // ? SAFETY
    let combined: Combined = serde_json::from_str(&combined_json).unwrap();
    with_mut_state(|s| s.combined_update(combined))
}
#[ic_cdk::update]
fn combined_increment_called(anchor: String) {
    let id = anchor.as_str().try_into();
    if let Ok(id) = id {
        let _ = with_mut_state(|s| s.combined_increment_called(id));
    }
}
#[ic_cdk::query]
fn combined_query(anchor: String) -> Option<String> {
    let id: CombinedParsedId = anchor.as_str().try_into().ok()?;
    with_state(|s| s.combined_query(id)).and_then(|combined| serde_json::to_string(&combined).ok())
}

// ================== dapp ==================

#[ic_cdk::update(guard = "must_be_admin")]
fn dapp_update(dapp_json: String) {
    #[allow(clippy::unwrap_used)] // ? SAFETY
    let dapp: Dapp = serde_json::from_str(&dapp_json).unwrap();
    with_mut_state(|s| s.dapp_update(dapp))
}
#[ic_cdk::update(guard = "must_be_admin")]
fn dapp_increment_called_by_admin(anchor: String) {
    let id: Result<DappParsedId, _> = anchor.as_str().try_into();
    if let Ok(id) = id {
        let _ = with_mut_state(|s| s.dapp_increment_called_by_admin(id));
    }
}
#[ic_cdk::update(guard = "must_be_admin")]
fn dapp_update_collected(anchor: String, collected: u64) {
    let id: Result<DappParsedId, _> = anchor.as_str().try_into();
    if let Ok(id) = id {
        let _ = with_mut_state(|s| s.dapp_update_collected(id, collected));
    }
}
#[ic_cdk::query(guard = "must_be_admin")]
fn dapp_query_by_admin(anchor: String) -> Result<String, String> {
    let id: DappParsedId = anchor.as_str().try_into()?;
    let dapp = with_state(|s| s.dapp_query_by_admin(id))?;
    serde_json::to_string(&dapp).map_err(|e| format!("serialize failed: {e}"))
}

// get access
#[ic_cdk::query]
fn dapp_query_access(anchor: String) -> Result<String, String> {
    let id: DappParsedId = anchor.as_str().try_into()?;
    let access = with_state(|s| s.dapp_query_access(id))?;
    let access = serde_json::to_string(&access).map_err(|err| format!("serialize access failed: {err}"))?;
    Ok(access)
}
#[ic_cdk::update]
fn dapp_increment_called_by_token(anchor: String, verified: Option<String>) {
    let id: Result<DappParsedId, _> = anchor.as_str().try_into();
    let verified = match verified {
        Some(verified) => {
            let verified: DappVerified = match serde_json::from_str(&verified) {
                Ok(verified) => verified,
                Err(_) => return,
            };
            Some(verified)
        }
        None => None,
    };
    if let Ok(id) = id {
        let _ = with_mut_state(|s| s.dapp_increment_called_by_token(id, verified));
    }
}
#[ic_cdk::query]
fn dapp_query_by_token(anchor: String, verified: Option<String>) -> Result<String, String> {
    let id: DappParsedId = anchor.as_str().try_into()?;
    let verified = match verified {
        Some(verified) => {
            let verified: DappVerified = match serde_json::from_str(&verified) {
                Ok(verified) => verified,
                Err(err) => return Err(format!("wrong verified: {err}")),
            };
            Some(verified)
        }
        None => None,
    };
    let dapp = with_state(|s| s.dapp_query_by_token(id, verified))?;
    serde_json::to_string(&dapp).map_err(|e| format!("serialize failed: {e}"))
}

// ================== common ==================

#[ic_cdk::query]
pub fn wallet_balance() -> candid::Nat {
    candid::Nat::from(ic_cdk::api::canister_balance128())
}

#[ic_cdk::update]
async fn canister_status() -> ic_cdk::api::management_canister::main::CanisterStatusResponse {
    #[allow(clippy::unwrap_used)] // ? SAFETY
    ic_cdk::api::management_canister::main::canister_status(ic_cdk::api::management_canister::main::CanisterIdRecord {
        canister_id: ic_cdk::api::id(),
    })
    .await
    .unwrap()
    .0
}

#[ic_cdk::query]
async fn whoami() -> Principal {
    ic_cdk::api::caller()
}

// ================== test ==================

#[ic_cdk::query]
#[cfg(not(test))]
fn __get_candid_interface_tmp_hack() -> String {
    candid::export_service!();
    __export_service()
}
