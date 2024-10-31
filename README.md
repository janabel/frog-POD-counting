# Frog Counting

Frog Counting will consist of two websites built on the [Zupass](https://github.com/proofcarryingdata/zupass) cryptographic data manager: [Frog Counter](https://frog-counter-8no1ly6fg-janabels-projects.vercel.app) and [Frog Whisperer Exchange](https://frog-whisperer-exchange-529xxypqi-janabels-projects.vercel.app).

The user may first navigate to Frog Counter, which will allow the user to i) fetch their frog PODs from Zupass and ii) generate a proof that says “I have N frog PODs”. By using the [Sonobe](https://github.com/privacy-scaling-explorations/sonobe) folding schemes library, this "N" can be larger than if we used a single circuit to prove ownership of all PODs at once.

Upon creating a proof with Frog Counter, the user may then navigate to the Frog Whisperer Exchange site. This second site will ultimately issue the user `FrogWhisperer` PODs certifying ownership of many Frog PODs from [FrogCrypto](https://frogcrypto.vercel.app/).

These websites together form a proof-of-concept app demonstrating the theoretical power of folding schemes and using the POD cryptographic data type. By combining the two, we have essentially found a way to turn PODs into a (private) currency -- one can both privately and verifiably trade in many PODs of one type into a POD of another type. Private because we don't need to reveal information about the PODs themselves (besides currently the message hash, see more in "Future Work"), and verifiable because the Frog Whisperer Exchange site verifies a folding proof.

---

# Local Dev

To run the Frog Counter (prover site), cd into the prover-site and then type the following into the CLI:
```
pnpm install
pnpm dev
```
To run the Frog Whisperer Exchange Exchange (verifier site), cd into the verifier-site and type the same commands as above.

---

# Preliminaries

## PODs

Frog PODs are pieces of data that include the following:

- <u>POD `entries`</u>: this is a JSON that includes attributes about the frog like `frogID`, `biome`, `speed`, `timestamp_signed`, etc. The JSON also includes a `ownerSemaphoreID`, the public semaphore ID of the owner of the POD, which was included by the authority that issued the frog POD (FrogCrypto).
- POD `signature`: this is a signature of the contentID of the POD entries. This contentID is the root of the merkle tree with hashes of the POD's entries as leaves.
- POD `signerPublicKey`: this is the public key of the authority that issued the POD (FrogCrypto).

## Folding schemes

Folding schemes are one approach to Incrementally Verifiable Computation (IVC), where one party proves correct execution of some computation with multiple "steps", where there is some new "state" associated with each step. Here, we are proving that the prover computed some $z_n = F(...~F(F(F(F(z_0, w_0), w_1), w_2), ...), w_{n-1})$ correctly, where $z_i = F(...~F(F(F(F(z_0, w_0), w_1), w_2), ...), w_{i-1})$ are the intermediate states for $i = 1, ..., n$ and $w_i$ are the external witnesses used at each iterative step.

![folding-diagram](https://camo.githubusercontent.com/e1bc3ebf5522471b6ea702146cb687e66aa5e8c272a121371d53905d33eebc97/68747470733a2f2f707269766163792d7363616c696e672d6578706c6f726174696f6e732e6769746875622e696f2f736f6e6f62652d646f63732f696d67732f666f6c64696e672d6d61696e2d696465612d6469616772616d2e706e67)

In this case, our intermediate state is the number of PODs we have verified so far (essentially a counter for the frog PODs). The function $F$ is a binary check for each frog POD; $F$ evaluates to $1$ if the frog POD valid and $0$ otherwise. The external witnesses $w_i$ are the frog PODs themselves that $F$ checks.

With folding schemes, instead of having to prove the computation of all iterations of $F$ in a traditional SNARK, the folding scheme will batch together a random linear combination of the individual witnesses $w_i$. The prover can then feed this batched witness directly into a (modified) version of the original R1CS checks for $F$ to provide an uncheatable proof, thus shortening the proving time.

---

# Website Flow

## Frog Counter (Prover)
- Website connects to user's Zupass 
- Gets an array of `Frog` PODs from Zupass using the new [Zupass](https://github.com/proofcarryingdata/zupass) API
- Generates IVC circuit inputs
  - Create frog inputs to pass into the circuit, which have:
    -  all frog POD entries
    - `semaphoreIDCommitment` fetched from Zupass
    - `frogSignerPubkey` of frog POD issuer 
    - `frogSignature` from frog POD issuer
  - Hardcodes (not useful) `watermark=2718`, `externalNullifier=STATIC_ZK_EDDSA_FROG_PCD_NULLIFIER`
- Generates an IVC proof with the circuit inputs above using Sonobe
- Outputs the Sonobe `IVCProof`, which the user can download as a serialized binary file

## Frog Whisperer Exchange (Verifier)
- Website connects to user's Zupass 
- User uploads `ivc_proof.bin` with upload button
- Site verifiers ivc_proof and also extracts the final IVC state, which includes the total number of frogs
- Create a `FrogWhisperer` POD to user's Zupass, which contains:
    - `num_frogs` = # of frogs that user owns
    - `ownerSemaphoreID` as the user’s public semaphore ID)
    - other fun entries like a picture of the Frog Whisperer Exchange, description of the POD, type of POD, etc.
    - signature of POD contentID with website’s `verifier_sk`
- Issue the POD to user’s Zupass “FrogWhisperer” folder

---

# Future Work

## Hiding the final contentID 

One potential issue is that a user may try to put multiple copies of a frog POD in order to prove they have more frogs than they actually do. Currently, the way that we guarantee uniqueness of the frog PODs is by sorting them by their contentID (the merkle root of hashed POD entries), and then checking that the contentIDs within the IVC circuit are in strictly increasing order. The sorting happens within the browser, but it is important to clarify that even if we can't trust our website to do, the verifier can be sure that the user cannot generate a valid proof if their frogs are not sorted. 

The way that we check that the contentIDs of the PODs are in increasing order is by including them in the state of the IVC computation, so that we can compare the contentID of each new POD folded in with the previous state, i.e. the previous POD's contentID. However, the final state of the IVC is public, so this would also reveal the contentID of the final POD. Normally this would not be an issue over completely random input PODs, but there may be a very small number of possible frog (or other) PODs in the first place. Thus, an adversary trying to gain information about the PODs included in the IVC proof only needs to brute force through a small number of PODs to determine what the original final POD was.

This issue would be solved if we used a full Decider proof instead of just extracting the IVC Proof, but that requires further optimizations before it can run without crashing in the browser.

## Ongoing/Future Performance Optimizations

## Benchmarks 
| Step                                      | Duration                     |
|-------------------------------------------|------------------------------|
| read in/create `external_inputs`          | 1.5 ms                       |
| create `circuit`                          | 21.0 ms                      |
| parameter deserialization `nova_pp`       | 81 ms                        |
| parameter deserialization `nova_vp`       | 6071.2 ms ~ 6.1 s           |
| create `nova_params`                      | 0.0 ms                       |
| initialize `nova`                         | 6703 ms ~ 6.7 s [~ x4 from CLI] |
| `Nova::prove_step` 1                      | 7774.6 ms ~ 7.7 s [~ x4]    |
| `Nova::prove_step` 2                      | 15942.5 ms ~ 15.9 s [~ x7]  |
| `Nova::prove_step` 3                      | 29849.1 ms ~ 29.8 s [~ x13] |
| `Nova::prove_step` 4                      | 43623.1 ms ~ 43.6 s [~ x17] |
| rest of folding                           | 6703.6 ms ~ 6.7 s           |
| serializing `proof`                       | 45.5 ms                      |
| **whole thing**                           | 58506.3 ms ~ 1 minute       |

Currently there are several slow steps within folding in the browser:
- <u>Witness generation</u>: Currently, the Sonobe library takes in a Web Assembly (WASM) file as instructions for witness generation at each IVC step. Because we then compile the API from Sonobe into wasm for our web app, our website is now simulating WASM within WASM. Carlos from the PSE team and a key contributor of the Sonobe library did wonderful work on modifying snarkjs to allow us to generate witnesses outside of WASM, as well as modifying Sonobe to be able to take in all serialized witnesses directly for the folding. With these new APIs, the runtimes for each `prove_step` are still similar in the CLI but we are hopeful that here is less overhead when compiling to WASM. We are currently working on integrating these into the project.
- <u>Parameter serialization</u>: serializing the `nova_vp` takes 6 seconds, but perhaps there are optimizations we can do with serializing parameters in WASM with Web Workers.
