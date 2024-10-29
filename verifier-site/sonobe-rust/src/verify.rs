// FROG FOLDING!

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]

use wasm_bindgen::prelude::*;
use web_sys::{window, Performance};
use wasm_bindgen::JsCast;

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
        decider_eth::{prepare_calldata, Decider as DeciderEth, VerifierParam},
        Nova, PreprocessorParam,
        ProverParams, VerifierParams, IVCProof
    },
    frontend::{circom::CircomFCircuit, FCircuit},
    transcript::poseidon::poseidon_canonical_config,
    Decider, FoldingScheme,
};

use std::path::PathBuf;
use std::time::Instant;
use serde_wasm_bindgen::to_value;
use std::fs;
use serde::Deserialize;
use std::collections::HashMap;

use std::panic::{self, AssertUnwindSafe};
use web_sys::console;

use ark_ff::PrimeField;
// use ark_ff::fields::{Fp256, MontBackend, MontConfig}
// use ark_bn254::FqConfig;

////////////////////////////////////////////////////////////////

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> f64;
}

fn get_current_time_in_millis() -> f64 {
    let window = window().expect("should have a window in this context");
    let performance = window.performance().expect("performance should be available");
    performance.now()
}

fn scalar_to_javascript(fq_element: Fr) -> String {
    let bigint_rep = fq_element.into_bigint();
    bigint_rep.to_string()
}

#[wasm_bindgen]
pub fn verifyRust(r1cs_bytes: Vec<u8>, 
    wasm_bytes: Vec<u8>,
    ivc_proof_serialized: Vec<u8>, 
    nova_pp_serialized: Vec<u8>, 
    nova_vp_serialized: Vec<u8>) 
    // -> Vec<G1::ScalarField> 
    -> String 
    {

    pub type N =
        Nova<G1, GVar, G2, GVar2, CircomFCircuit<Fr>, KZG<'static, Bn254>, Pedersen<G2>, false>;

    let start_total = get_current_time_in_millis();

    // create f_circuit_params (necessary for deserializing params)
    let start = get_current_time_in_millis();
        let f_circuit_params = (r1cs_bytes.into(), wasm_bytes.into(), 3, 22);

        let end = get_current_time_in_millis();
        let elapsed = end - start;
        web_sys::console::log_1(&format!("created f_circuit_params: {:?}", elapsed).into());

    // deserialize ivc_proof
    let start = get_current_time_in_millis();
        let ivc_proof = IVCProof::deserialize_uncompressed(ivc_proof_serialized.as_slice()).unwrap();

        let end = get_current_time_in_millis();
        let elapsed = end - start;
        web_sys::console::log_1(&format!("deserialized ivc_proof: {:?}", elapsed).into());

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
        web_sys::console::log_1(&format!("deserialized nova_vp: {:?}", elapsed).into());

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


    let start = get_current_time_in_millis();
        let nova_params = (nova_pp_deserialized, nova_vp_deserialized);
        let end = get_current_time_in_millis();
        let elapsed = end - start;
        web_sys::console::log_1(&format!("created nova_params {:?}", elapsed).into());

    let start = get_current_time_in_millis();
        web_sys::console::log_1(&format!("verifying proof...").into());
        assert!(N::verify(nova_params.1.clone(), ivc_proof.clone()).is_ok());
        let end = get_current_time_in_millis();
        let elapsed = end - start;
        web_sys::console::log_1(&format!("🎉🎉🎉 verified proof!! {:?}", elapsed).into());

    let end_total = get_current_time_in_millis();
    let elapsed_total = end_total - start_total;

    web_sys::console::log_1(&format!("whole thing took this long: {:?}", elapsed_total).into());

    web_sys::console::log_1(&format!("end state for ivc z_i: {:?}", ivc_proof.z_i).into());
    // let mut z_i2_serialized = Vec::new();
    // (ivc_proof.z_i)[2].serialize(&mut serialized).unwrap(); // Serialize each scalar
    
    let z_i2_serialized = scalar_to_javascript((ivc_proof.z_i)[2]);
    web_sys::console::log_1(&format!("end SERIALIZED state for ivc z_i: {:?}", z_i2_serialized).into());
    return z_i2_serialized;

    // return ivc_proof.z_i;

}