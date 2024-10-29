// this file reads in serialized ivc proof, deserializes, and verifies

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

use ark_bn254::{constraints::GVar, Bn254, Fr, G1Projective as G1};
use ark_grumpkin::{constraints::GVar as GVar2, Projective as G2};
// use pasta curves b/c no more onchain stuff
// use ark_pallas::{constraints::GVar, Affine, Fr, Projective as G1};
// use ark_vesta::{constraints::GVar as GVar2, Projective as G2};

use folding_schemes::commitment::CommitmentScheme;
use folding_schemes::{
    commitment::{kzg::KZG, pedersen::Pedersen},
    folding::nova::{
        decider_eth::{prepare_calldata, Decider as DeciderEth, VerifierParam},
        Nova, PreprocessorParam,
        ProverParams, VerifierParams, IVCProof,
    },
    frontend::{FCircuit},
    transcript::poseidon::poseidon_canonical_config,
    Decider, FoldingScheme,
};
use frontends::{circom::CircomFCircuit};

// use folding_schemes::folding::TestPrint;

use std::path::PathBuf;
use std::time::Instant;
use std::fs::File;
use std::io::Read;

// helper function to read in values from files (e.g. frog inputs and serialized parameters)
fn read_binary_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    // Create a buffer to hold the binary data
    let mut buffer = Vec::new();
    // Read the file's binary data into the buffer
    file.read_to_end(&mut buffer).unwrap();
    return buffer;
}

fn main() {
    let start_total = Instant::now();

    // TestPrint::test_print();

    let nova_pp_serialized = read_binary_file("./serialized_outputs/uncompressed_IVCProof_only/nova_pp_output_pedersen.bin");
    let nova_vp_serialized = read_binary_file("./serialized_outputs/uncompressed_IVCProof_only/nova_vp_output_pedersen.bin");
    println!("{}", "succesfully read nova serialized params!");

    let ivc_proof_serialized = read_binary_file("./serialized_outputs/nova_ivc_proof_pedersen.bin");
    println!("{}", "succesfully read serialized nova IVC_proof!");

    // initialize the Circom circuit (needed to deserialize verifier params)
    let r1cs_path = PathBuf::from(
        "./circuits/build/frogIVC.r1cs"
    );
    let wasm_path = PathBuf::from(
        "./circuits/build/frogIVC_js/frogIVC.wasm",
    );

    // (r1cs_bytes, wasm_bytes, state_len, external_inputs_len)
    let start = Instant::now();
        let f_circuit_params = (r1cs_path.into(), wasm_path.into(), 3, 21);
        println!("created circuit params!: {:?}", start.elapsed());

    pub type N =
        Nova<G1, GVar, G2, GVar2, CircomFCircuit<Fr>, Pedersen<G1>, Pedersen<G2>, false>;

    // read in the nova params so don't need to preprocess them
    let start = Instant::now();
        let nova_pp_deserialized = N::pp_deserialize_with_mode(
            &mut nova_pp_serialized.as_slice(),
            ark_serialize::Compress::No,
            ark_serialize::Validate::No,
            f_circuit_params.clone(), // f_circuit_params 
        )
        .unwrap();
        // let nova_pp_deserialized = ProverParams::<
        //     G1,
        //     G2,
        //     Pedersen<G1>,
        //     Pedersen<G2>,
        //     >::deserialize_with_mode(
        //         &mut nova_pp_serialized.as_slice(),
        //         ark_serialize::Compress::No,
        //         ark_serialize::Validate::No,
        //         fcircuit_params.clone()
        //     )
        //     .unwrap();
        println!("deserialized nova_pp: {:?}", start.elapsed());

    let start = Instant::now();
        let nova_vp_deserialized = N::vp_deserialize_with_mode(
            &mut nova_vp_serialized.as_slice(),
            ark_serialize::Compress::No,
            ark_serialize::Validate::No,
            f_circuit_params.clone(), // f_circuit_params 
        )
        .unwrap();
        // let nova_vp_deserialized = VerifierParams::<
        //         G1,
        //         G2,
        //         Pedersen<G1>,
        //         Pedersen<G2>,
        //         false,
        //     >::deserialize_with_mode::<GVar, GVar2, CircomFCircuit<Fr>, _>(
        //         &mut nova_vp_serialized.as_slice(),
        //         ark_serialize::Compress::No,
        //         ark_serialize::Validate::No,
        //         f_circuit_params,
        //     )
        //     .unwrap();
        println!("deserialized nova_vp: {:?}", start.elapsed());

    println!("{}", "successfully serialized all nova params!");

    let ivc_proof = IVCProof::deserialize_uncompressed(ivc_proof_serialized.as_slice()).unwrap();

    println!("{}", "succesfully deserialized nova ivc_proof");

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    let start = Instant::now();
        let nova_params = (nova_pp_deserialized, nova_vp_deserialized);
        println!("created nova_params: {:?}", start.elapsed());

    let start = Instant::now();
        // let verify_result = N::verify(nova_params.1.clone(), ivc_proof.clone()).unwrap();
        // println!("verify_result = {:?}", verify_result);
        assert!(N::verify(nova_params.1.clone(), ivc_proof.clone()).is_ok());
        println!("verify ivc_proof: {:?}", start.elapsed());

    println!("whole thing took this long: {:?}", start_total.elapsed());


}