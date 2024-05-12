use rand::{CryptoRng, RngCore, SeedableRng};
use ic_cdk::api::trap;
use rand_chacha::ChaCha20Rng;
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::call;



async fn make_rng<T: SeedableRng + CryptoRng>() -> T
where
    // raw_rand returns 32 bytes
    T: SeedableRng<Seed = [u8; 32]>,
{
    let raw_rand: Vec<u8> = match call(Principal::management_canister(), "raw_rand", ()).await {
        Ok((res,)) => res,
        Err((_, err)) => trap(&format!("failed to get seed: {}", err)),
    };

    let seed: <T as SeedableRng>::Seed = raw_rand[..].try_into().unwrap_or_else(|_| {
        trap(&format!(
            "when creating seed from raw_rand output, expected raw randomness to be of length 32, got {}",
            raw_rand.len()
        ));
    });

    T::from_seed(seed)
}


pub async fn create_id() -> u32{
    let mut rng: ChaCha20Rng = make_rng().await;
    return rng.next_u32();
}