// THIS FILE ORIGINALLY TESTED SERIALIZATION OF proof, decider_vp, AND public_inputs (just printed, didn't write to files yet)
// ON BRANCH verifier-site, TESTED FEEDING THESE SERIALIZED INTO THE VERIFIER WORKS
// NOW IT IS USED FOR SERIALIZING PP/VP AND WRITING TO FILES

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
// use ark_bn254::Bn254;
use ark_crypto_primitives::sponge::Absorb;
use ark_ec::{AffineRepr, CurveGroup, Group};
use ark_ff::{BigInteger, PrimeField};
// use ark_groth16::Groth16;
use ark_r1cs_std::{groups::GroupOpsBounds, prelude::CurveVar, ToConstraintFieldGadget};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_snark::SNARK;
use ark_std::rand::{CryptoRng, RngCore};
use ark_std::{One, Zero};
use core::marker::PhantomData;
// use ark_ff::MontBackend;
///
/// This example performs the full flow:
/// - define the circuit to be folded
/// - fold the circuit with Nova+CycleFold's IVC
/// - generate a DeciderEthCircuit final proof
/// - generate the Solidity contract that verifies the proof
/// - verify the proof in the EVM
///
/// 
/// 
// use wasm_bindgen::prelude::wasm_bindgen;

// use ethereum_types::U256;
use ark_bn254::{constraints::GVar, Bn254, Fr, G1Projective as G1};

// use ark_groth16::Groth16;
use ark_groth16::{Groth16, VerifyingKey, ProvingKey};
// use ark_groth16::{Groth16, Proof};
use ark_grumpkin::{constraints::GVar as GVar2, Projective as G2};

use num_bigint::BigInt;
use num_traits::Num;
use ark_ff::BigInteger256;

use std::path::PathBuf;
use std::time::Instant;

use folding_schemes::folding::nova::CommittedInstance;
use folding_schemes::{
    commitment::{kzg::KZG, pedersen::Pedersen},
    folding::nova::{
        decider_eth::{prepare_calldata, Decider as DeciderEth, VerifierParam, Proof},
        Nova, PreprocessorParam,
        ProverParams, VerifierParams,
    },
    frontend::{circom::CircomFCircuit, FCircuit},
    transcript::poseidon::poseidon_canonical_config,
    Decider, FoldingScheme,
};

use std::fs;
// use std::io::Result;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
// use serde_json::Value;
use std::collections::HashMap;

// rng: we'll need to use a deterministic seed on both proof and verification
// let's just set it to 0
// need an rng that supports initializing from a seed ==> chacha20rng
// it also implements RngCore and CryptoRng traits (which are required by nova/decider) yay :)
use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;

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
    semaphoreIdentityTrapdoor: String,
    semaphoreIdentityNullifier: String,
    watermark: String,
    frogSignatureR8x: String,
    frogSignatureR8y: String,
    frogSignatureS: String,
    externalNullifier: String,
    reservedField1: String,
    reservedField2: String,
    reservedField3: String,
}

fn main() {

    println!("\nlet's 🐸 fold 🐸 some 🐸 frogs 🐸🐸🐸🐸🐸\n");

    let file_path = "./src/frog_inputs (4).json";
    let contents = fs::read_to_string(file_path);
    let frogs: HashMap<String, Frog> = serde_json::from_str(&contents.unwrap()).unwrap();

    // helper function to turn strings into Fr elements to feed into folding/circuits
    fn str_to_fr(input_string: &str)-> Fr {
        // let bigint: BigInt;
        // if (&input_string[0..2] == "0x") {
        //     bigint = BigInt::from_str_radix(input_string, 16).unwrap();
        // } else {
        //     bigint = BigInt::from_str_radix(input_string, 10).unwrap();
        // }
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
            str_to_fr(&frog.timestampSigned),
            str_to_fr(&frog.ownerSemaphoreId),
            str_to_fr(&frog.frogSignerPubkeyAx),
            str_to_fr(&frog.frogSignerPubkeyAy),
            str_to_fr(&frog.semaphoreIdentityTrapdoor),
            str_to_fr(&frog.semaphoreIdentityNullifier),
            str_to_fr(&frog.watermark),
            str_to_fr(&frog.frogSignatureR8x),
            str_to_fr(&frog.frogSignatureR8y),
            str_to_fr(&frog.frogSignatureS),
            str_to_fr(&frog.externalNullifier),
            str_to_fr(&frog.biome),
            str_to_fr(&frog.rarity),
            str_to_fr(&frog.temperament),
            str_to_fr(&frog.jump),
            str_to_fr(&frog.speed),
            str_to_fr(&frog.intelligence),
            str_to_fr(&frog.beauty),
            str_to_fr(&frog.reservedField1),
            str_to_fr(&frog.reservedField2),
            str_to_fr(&frog.reservedField3),
        ];
    }

    let mut external_inputs: Vec<Vec<Fr>> = Vec::new();
    let n = frogs.len();

    for i in 1..=n {
        let i_string = i.to_string();
        let frog = &frogs[&i_string];
        let frog_fr_vector = frog_to_fr_vector(&frog);
        external_inputs.push(frog_fr_vector);
    }

    // println!("printing external_inputs...");
    // println!("{:?}", external_inputs);

    // set the initial state
    // initialize z_0 to [0,0,0] (to compare against any first [frogMessageHash2Small_fr, frogMessageHash2Big_fr])
    let z_0 = vec![Fr::from(0_u32), Fr::from(0_u32), Fr::from(0_u32)]; 

    // set the external inputs to be used at each step of the IVC
    // external_input is just two frogs, formatted correctly

    // initialize the Circom circuit
    let r1cs_path = PathBuf::from(
        "./circuits/build/frogIVC.r1cs"
    );
    let wasm_path = PathBuf::from(
        "./circuits/build/frogIVC_js/frogIVC.wasm",
    );

    // (r1cs_path, wasm_path, state_len, external_inputs_len)
    let f_circuit_params = (r1cs_path.into(), wasm_path.into(), 3, 22);
    let f_circuit = CircomFCircuit::<Fr>::new(f_circuit_params).unwrap();

    println!("{}", "created circuit!");

    pub type N =
        Nova<G1, GVar, G2, GVar2, CircomFCircuit<Fr>, KZG<'static, Bn254>, Pedersen<G2>, false>;
    pub type D = DeciderEth<
        G1,
        GVar,
        G2,
        GVar2,
        CircomFCircuit<Fr>,
        KZG<'static, Bn254>,
        Pedersen<G2>,
        Groth16<Bn254>,
        N,
    >;

    let poseidon_config = poseidon_canonical_config::<Fr>();
    let mut rng = rand::rngs::OsRng;
    // let mut rng = ChaCha20Rng::from_seed([0u8; 32]);

    // prepare the Nova prover & verifier params
    let nova_preprocess_params = PreprocessorParam::new(poseidon_config, f_circuit.clone());
    let nova_params = N::preprocess(&mut rng, &nova_preprocess_params).unwrap(); // calls KZG10 trusted setup

    println!("{}", "prepared nova prover and verifier params!");

    // initialize the folding scheme engine, in our case we use Nova
    let mut nova = N::init(&nova_params, f_circuit.clone(), z_0).unwrap();

    println!("{}", "initialized folding scheme!");

    // prepare the Decider prover & verifier params
    let (decider_pp, decider_vp) = D::preprocess(&mut rng, nova_params.clone(), nova.clone()).unwrap();
    // println!("generated decider_pp: {:?}", decider_pp);
    println!("generated decider_vp: {:?}", decider_vp);

    println!("{}", "prepared decider prover & verifier params!");

    // serialize the Nova params. These params are the trusted setup of the commitment schemes used
    // (ie. KZG & Pedersen in this case)

            // let pp_hash = nova_vp.pp_hash()?;
            // let pp = (g16_pk, nova_pp.cs_pp);
            //     let vp = Self::VerifierParam {
            //         pp_hash,
            //         snark_vp: g16_vk,
            //         cs_vp: nova_vp.cs_vp,
            //     };

    // serialize nova_pp and nova_vp
    let mut nova_pp_serialized = vec![];
        nova_params
            .0
            .serialize_with_mode(&mut nova_pp_serialized, ark_serialize::Compress::No)
            .unwrap();
        let mut nova_vp_serialized = vec![];
        nova_params
            .1
            .serialize_with_mode(&mut nova_vp_serialized, ark_serialize::Compress::No)
            .unwrap();
    
    // write all serialized parameters to output files
    let mut file_nova_pp = File::create("./serialized_outputs/uncompressed_new/nova_pp_output.bin").unwrap();
        file_nova_pp.write_all(&nova_pp_serialized).unwrap();
        println!("nova_pp written to nova_pp_output.bin");
    let mut file_nova_vp = File::create("./serialized_outputs/uncompressed_new/nova_vp_output.bin").unwrap();
        file_nova_vp.write_all(&nova_vp_serialized).unwrap();
        println!("nova_vp written to nova_vp_output.bin");

    // run n steps of the folding iteration
    for (i, external_inputs_at_step) in external_inputs.iter().enumerate() {
        let start = Instant::now();
        nova.prove_step(&mut rng, external_inputs_at_step.clone(), None)
            .unwrap(); //////////////////////////////////////////////////////////////////////////////////////////
        println!("🐸 Nova::prove_step {}: {:?}", i, start.elapsed());
        println!(
            "state at last step (after {} iterations): {:?}",
            i,
            nova.state()
        )
    }

    println!("{}", "finished folding!");

    let start = Instant::now();
    let proof = D::prove(&mut rng, decider_pp, nova.clone()).unwrap();
    // println!("{:?}", proof);

    println!("generated Decider proof: {:?}", start.elapsed());

    let verified = D::verify(
        decider_vp.clone(),
        nova.i,
        nova.z_0.clone(),
        nova.z_i.clone(),
        &nova.U_i,
        &nova.u_i,
        &proof,
    )
    .unwrap();
    println!("unwrapped verify: {:?}", start.elapsed());

    assert!(verified);
    println!("Decider proof verification: {}", verified);

    println!("🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉");
    println!("🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉");


    // example of serializing the decider_vp, proof, and public_inputs 
    // (and printing them to use for verifier testing)

    let mut decider_vp_serialized = vec![];
        decider_vp
            .serialize_with_mode(&mut decider_vp_serialized, ark_serialize::Compress::No)
            .unwrap();
        let mut proof_serialized = vec![];
        proof.serialize_with_mode(&mut proof_serialized, ark_serialize::Compress::No).unwrap();
        // serialize the public inputs in a single packet
        let mut public_inputs_serialized = vec![];
        nova.i
            .serialize_with_mode(&mut public_inputs_serialized, ark_serialize::Compress::No)
            .unwrap();
        nova.z_0
            .serialize_with_mode(&mut public_inputs_serialized, ark_serialize::Compress::No)
            .unwrap();
        nova.z_i
            .serialize_with_mode(&mut public_inputs_serialized, ark_serialize::Compress::No)
            .unwrap();
        nova.U_i
            .serialize_with_mode(&mut public_inputs_serialized, ark_serialize::Compress::No)
            .unwrap();
        nova.u_i
            .serialize_with_mode(&mut public_inputs_serialized, ark_serialize::Compress::No)
            .unwrap();

        println!("{}", "succesfully serialized proof and decider_vp");

        println!("{:?}", decider_vp_serialized);
        println!("{:?}", proof_serialized);
        println!("{:?}", public_inputs_serialized);

    
}