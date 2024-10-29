// FROG FOLDING!

use wasm_bindgen::prelude::*;
use web_sys::{window, Performance};
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> f64;
}

// #[wasm_bindgen]
fn get_current_time_in_millis() -> f64 {
    let window = window().expect("should have a window in this context");
    let performance = window.performance().expect("performance should be available");
    performance.now()
}

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_groth16::{Groth16, VerifyingKey, ProvingKey};
use ark_bn254::{constraints::GVar, Bn254, Fr, G1Projective as G1};
use ark_grumpkin::{constraints::GVar as GVar2, Projective as G2};

use num_bigint::BigInt;
use num_traits::Num;
use ark_ff::BigInteger256;

use folding_schemes::commitment::CommitmentScheme;
use folding_schemes::{
    commitment::{kzg::KZG, pedersen::Pedersen},
    folding::nova::{
        Nova, PreprocessorParam,
        ProverParams, VerifierParams,
    },
    frontend::{circom::CircomFCircuit, FCircuit},
    transcript::poseidon::poseidon_canonical_config, FoldingScheme,
};
// use frontends::circom::CircomFCircuit, 

use std::path::PathBuf;
use std::time::Instant;
use serde_wasm_bindgen::to_value;
use std::fs;
use serde::Deserialize;
use std::collections::HashMap;

// RNG: we'll need to use a deterministic seed on both proof and verification
// (because parameter preprocessing and prove/verify depend on it)
// let's just set seed to 0 for now
// need an rng that supports initializing from a seed ==> chacha20rng
// it also implements RngCore and CryptoRng traits (which are required by nova/decider) yay :)
use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;

use std::panic::{self, AssertUnwindSafe};
use web_sys::console;

fn set_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };

        console::error_1(&format!("Panic occurred: {}", msg).into());
    }));
}


#[derive(Debug, Deserialize)]
struct Frog {
    frogId: String,
    biome: String,
    rarity: String,
    temperament: String,
    jump: String,
    speed: String,
    intelligence: String,
    beauty: String,
    timestampSigned: String,
    ownerSemaphoreId: String,
    frogSignerPubkeyAx: String,
    frogSignerPubkeyAy: String,
    semaphoreIdentityCommitment: String,
    watermark: String,
    frogSignatureR8x: String,
    frogSignatureR8y: String,
    frogSignatureS: String,
    externalNullifier: String,
    reservedField1: String,
    reservedField2: String,
    reservedField3: String,
    contentID: String,
}

fn str_to_fr(input_string: &str)-> Fr {
    let bigint: BigInt = BigInt::from_str_radix(input_string, 10).unwrap();

    let bigint_bytes = bigint.to_bytes_be().1;
    let mut bytes_32 = vec![0_u8; 32];
    bytes_32[(32 - bigint_bytes.len())..].copy_from_slice(&bigint_bytes);

    let big_integer = BigInteger256::new([
        u64::from_be_bytes(bytes_32[24..32].try_into().unwrap()),
        u64::from_be_bytes(bytes_32[16..24].try_into().unwrap()),
        u64::from_be_bytes(bytes_32[8..16].try_into().unwrap()),
        u64::from_be_bytes(bytes_32[0..8].try_into().unwrap()),
    ]);

    return Fr::from(big_integer);
}

fn frog_to_fr_vector(frog: &Frog) -> Vec<Fr> {
    return vec![
        str_to_fr(&frog.frogId),
        str_to_fr(&frog.biome),
        str_to_fr(&frog.rarity),
        str_to_fr(&frog.temperament),
        str_to_fr(&frog.jump),
        str_to_fr(&frog.speed),
        str_to_fr(&frog.intelligence),
        str_to_fr(&frog.beauty),
        str_to_fr(&frog.timestampSigned),
        str_to_fr(&frog.ownerSemaphoreId),
        str_to_fr(&frog.frogSignerPubkeyAx),
        str_to_fr(&frog.frogSignerPubkeyAy),
        str_to_fr(&frog.semaphoreIdentityCommitment),
        str_to_fr(&frog.watermark),
        str_to_fr(&frog.frogSignatureR8x),
        str_to_fr(&frog.frogSignatureR8y),
        str_to_fr(&frog.frogSignatureS),
        str_to_fr(&frog.externalNullifier),
        str_to_fr(&frog.reservedField1),
        str_to_fr(&frog.reservedField2),
        str_to_fr(&frog.reservedField3),
        str_to_fr(&frog.contentID),
    ];
}

#[wasm_bindgen]
pub fn frog_nova(r1cs_bytes: Vec<u8>,
    wasm_bytes: Vec<u8>,
    frogs_js: JsValue,
    nova_pp_serialized: Vec<u8>,
    nova_vp_serialized: Vec<u8>,
    ) -> Vec<u8> {

    set_panic_hook();
    // alert("panic hook set!");

    pub type N =
        Nova<G1, GVar, G2, GVar2, CircomFCircuit<Fr>, KZG<'static, Bn254>, Pedersen<G2>, false>;
    
    let start_total = get_current_time_in_millis();
    
    let poseidon_config = poseidon_canonical_config::<Fr>();
    // let mut rng = rand::rngs::OsRng;
    let mut rng = ChaCha20Rng::from_seed([0u8; 32]); // need to use deterministic rng because downloading parameters (generated from same rng + seed)

    println!("Start frog_nova");

    let start = get_current_time_in_millis();
        let frogs_str = frogs_js.as_string().unwrap(); // String
        // let frogs: Vec<Frog> = serde_json::from_str(&frogs_str).expect("Failed to deserialize JSON");
        let frogs: HashMap<String, Frog> = serde_json::from_str(&frogs_str).unwrap();
        let mut external_inputs: Vec<Vec<Fr>> = Vec::new();
        let n = frogs.len();
        for i in 1..=n {
            let i_string = i.to_string();
            let frog = &frogs[&i_string];
            let frog_fr_vector = frog_to_fr_vector(frog);
            external_inputs.push(frog_fr_vector)
        }
        
        let end = get_current_time_in_millis();
        let elapsed = end - start;
        // alert("external_inputs created");
        web_sys::console::log_1(&format!("external_inputs created: {:?}", elapsed).into());

    // initialize z_0 to [0,0,0] (to compare against any first [frogMessageHash2Small_fr, frogMessageHash2Big_fr])
    let z_0 = vec![Fr::from(0_u32), Fr::from(0_u32), Fr::from(0_u32)]; 

    let start = get_current_time_in_millis();
        // (r1cs_bytes, wasm_bytes, state_len, external_inputs_len)
        let f_circuit_params = (r1cs_bytes.into(), wasm_bytes.into(), 3, 22);
        let mut f_circuit = CircomFCircuit::<Fr>::new(f_circuit_params.clone()).unwrap();
        
        let end = get_current_time_in_millis();
        let elapsed = end - start;
        // alert("created circuit!");
        web_sys::console::log_1(&format!("created circuits: {:?}", elapsed).into());

    // deserialize parameters
    let start = get_current_time_in_millis();
        // read in the nova params so don't need to preprocess them
        let nova_pp_deserialized = N::pp_deserialize_with_mode(
            &mut nova_pp_serialized.as_slice(),
            ark_serialize::Compress::No,
            ark_serialize::Validate::No,
            f_circuit_params.clone(), // f_circuit_params 
        )
        .unwrap();

        let end = get_current_time_in_millis();
        let elapsed = end - start;
        // alert("deserialized nova_pp");
        //  web_sys::console::log_1(&format!("nova_pp_deserialized: {:?}", nova_pp_deserialized).into());
         web_sys::console::log_1(&format!("deserialized nova_pp: {:?}", elapsed).into());

    let start = get_current_time_in_millis();
        let nova_vp_deserialized = N::vp_deserialize_with_mode(
            &mut nova_vp_serialized.as_slice(),
            ark_serialize::Compress::No,
            ark_serialize::Validate::No,
            f_circuit_params.clone(), // f_circuit_params 
        )
        .unwrap();

        let end = get_current_time_in_millis();
        let elapsed = end - start;
        // alert("deserialized nova_vp");
        //  web_sys::console::log_1(&format!("nova_vp_deserialized: {:?}", nova_vp_deserialized).into());
         web_sys::console::log_1(&format!("deserialized nova_vp: {:?}", elapsed).into());

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    let start = get_current_time_in_millis();
        let nova_params = (nova_pp_deserialized, nova_vp_deserialized);
        
        let end = get_current_time_in_millis();
        let elapsed = end - start;
        // alert("created nova_params");
        //  web_sys::console::log_1(&format!("nova_params: {:?}", nova_params).into());
         web_sys::console::log_1(&format!("created nova_params: {:?}", elapsed).into());

    let start = get_current_time_in_millis();
        let mut nova = N::init(&nova_params, f_circuit.clone(), z_0).unwrap();
        
        let end = get_current_time_in_millis();
        let elapsed = end - start;
        // alert("initialized nova from the deserialized parameters"); // WORKS IN BROWSER UP TO HERE AS OF 9/13 10:44 PM
         web_sys::console::log_1(&format!("initialized nova: {:?}", elapsed).into());

    // run n steps of the folding iteration
    let start_outer = get_current_time_in_millis();
        for (i, external_inputs_at_step) in external_inputs.iter().enumerate() {
            nova.prove_step(rng.clone(), external_inputs_at_step.clone(), None)
                .unwrap();  
            
            let end = get_current_time_in_millis();
            let elapsed = end - start_outer;
            // alert(&format!("Nova::prove_step {}", i));
            web_sys::console::log_1(&format!("Nova::prove_step {:?}", elapsed).into());
        }

        let end_outer = get_current_time_in_millis();
        let elapsed_outer = end_outer - start_outer;
        // alert("finished folding!"); // works up to here in browser 8/23 12:17PM
        web_sys::console::log_1(&format!("finished folding: {:?}", elapsed).into());

    let start = get_current_time_in_millis();
        
    // now serialize just the IVCProof
    let ivc_proof = nova.ivc_proof();

    let start = get_current_time_in_millis();
        let mut proof_serialized = vec![];
        ivc_proof.serialize_with_mode(&mut proof_serialized, ark_serialize::Compress::No).unwrap();

        let end = get_current_time_in_millis();
        let elapsed = end - start;
        // alert(&format!("proof_serialized: {:?}", proof_serialized));
        web_sys::console::log_1(&format!("serialized proof: {:?}", elapsed).into());
        web_sys::console::log_1(&format!("proof_serialized: {:?}", proof_serialized).into());

    let end_total = get_current_time_in_millis();
    let elapsed_total = end_total - start_total;
    web_sys::console::log_1(&format!("WHOLE THING TOOK: {:?}", elapsed_total).into());
    alert("🎉 Created proof!");

    return proof_serialized;
}