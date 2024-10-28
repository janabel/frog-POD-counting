// THIS FILE ORIGINALLY TESTED SERIALIZATION OF proof, decider_vp, AND public_inputs (just printed, didn't write to files yet)
// ON BRANCH verifier-site, TESTED FEEDING THESE SERIALIZED INTO THE VERIFIER WORKS
// NOW IT IS USED FOR SERIALIZING PP/VP AND WRITING TO FILES

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

use ark_bn254::{constraints::GVar, Bn254, Fr, G1Projective as G1};
use ark_grumpkin::{constraints::GVar as GVar2, Projective as G2};
// use pasta curves b/c no more onchain stuff
// use ark_pallas::{constraints::GVar, Affine, Fr, Projective as G1};
// use ark_vesta::{constraints::GVar as GVar2, Projective as G2};

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

use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

// rng: we'll need to use a deterministic seed on both proof and verification
// let's just set it to 0
// need an rng that supports initializing from a seed ==> chacha20rng
// it also implements RngCore and CryptoRng traits (which are required by nova/decider) yay :)
use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;

fn main() {

    println!("\nlet's 🐸 serialize 🐸 some 🐸 parameters 🐸🐸🐸🐸🐸\n");

    // initialize the Circom circuit
    let r1cs_path = PathBuf::from(
        "./circuits/build/frogIVC.r1cs"
    );
    let wasm_path = PathBuf::from(
        "./circuits/build/frogIVC_js/frogIVC.wasm",
    );

    // (r1cs_path, wasm_path, state_len, external_inputs_len)
    let f_circuit_params = (r1cs_path.into(), wasm_path.into(), 3, 21);
    let f_circuit = CircomFCircuit::<Fr>::new(f_circuit_params).unwrap();

    println!("{}", "created circuit!");

    pub type N =
        Nova<G1, GVar, G2, GVar2, CircomFCircuit<Fr>, KZG<'static, Bn254>, Pedersen<G2>, false>;

    let poseidon_config = poseidon_canonical_config::<Fr>();
    // let mut rng = rand::rngs::OsRng;
    let mut rng = ChaCha20Rng::from_seed([0u8; 32]);

    // prepare the Nova prover & verifier params
    let nova_preprocess_params = PreprocessorParam::new(poseidon_config, f_circuit.clone());
    let nova_params = N::preprocess(&mut rng, &nova_preprocess_params).unwrap(); // calls KZG10 trusted setup

    println!("{}", "prepared nova prover and verifier params!");

    // serialize the Nova params. These params are the trusted setup of the commitment schemes used
    // (ie. PEDERSEN & Pedersen in this case)
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
    let mut file_nova_pp = File::create("./serialized_outputs/uncompressed_IVCProof_only/nova_pp_output.bin").unwrap();
        file_nova_pp.write_all(&nova_pp_serialized).unwrap();
        println!("nova_pp written to uncompressed_IVCProof_only/nova_pp_output.bin");
    let mut file_nova_vp = File::create("./serialized_outputs/uncompressed_IVCProof_only/nova_vp_output.bin").unwrap();
        file_nova_vp.write_all(&nova_vp_serialized).unwrap();
        println!("nova_vp written to uncompressed_IVCProof_only/nova_vp_output.bin");

}