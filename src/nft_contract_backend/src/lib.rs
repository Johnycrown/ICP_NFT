use ic_cdk_macros::*;
use ic_cdk::storage;

// Define NFT struct
#[derive(Clone)]
struct NFT {
    owner: ic_types::PrincipalId,
    metadata: String,
    // Add any other fields you need
}

// Define NFT contract
#[init]
fn init() {
    let initial_owner = ic_cdk::caller(); // Get the principal ID of the caller (the canister creator)

}

#[query]
fn get_nft_metadata(token_id: u64) -> Option<String> {
    if let Some(nft) = storage::get::<NFT>(token_id) {
        Some(nft.metadata)
    } else {
        None
    }
}

#[update]
fn mint_nft(owner: ic_types::PrincipalId, token_id: u64, metadata: String) {
    // Mint a new NFT with the given owner, token ID, and metadata
    let new_nft = NFT {
        owner: owner.clone(),
        metadata: metadata.clone(),
    };
    storage::insert(token_id, new_nft);
}

#[update]
fn transfer_nft(sender: ic_types::PrincipalId, receiver: ic_types::PrincipalId, token_id: u64) {
    // Transfer ownership of the NFT from sender to receiver
    if let Some(mut nft) = storage::get::<NFT>(token_id) {
        if nft.owner == sender {
            nft.owner = receiver;
            storage::insert(token_id, nft);
        } else {
            ic_cdk::trap("Sender does not own the NFT");
        }
    } else {
        ic_cdk::trap("NFT does not exist");

    }
}
