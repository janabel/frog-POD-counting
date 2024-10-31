# Frog Counting

Frog Counting will consist of two websites built on Zupass: Frog Counter and Frog Whisperer.

The user may first navigate to Frog Counter, which will allow the user to i) fetch their frog PODs from Zupass and ii) generate a proof that says “I have N frog PODs”. By using the [Sonobe](https://github.com/privacy-scaling-explorations/sonobe) folding schemes library, this "N" can be larger than if we used a single circuit to prove ownership of all PODs at once.

Upon creating a proof with Frog Counter, the user may then navigate to the Frog Whisperer site. This second site will ultimately issue the user `FrogWhisperer` PODs certifying ownership of many Frog PODs from [FrogCrypto](https://frogcrypto.vercel.app/).

These websites together form a proof-of-concept app demonstrating the theoretical power of folding schemes and using the POD cryptographic data type. By combining the two, we have essentially found a way to turn PODs into a (private) currency -- one can both privately and verifiably trade in many PODs of one type into a POD of another type. Private because we don't need to reveal information about the PODs themselves.

There is still much work to be done to get folding to be fast in e.g. the browser.

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

In this case, our intermediate state is the number of PODs we have verified so far (a counter). The function $F$ is a binary check for each frog POD; $F$ evaluates to $1$ if the frog POD valid and $0$ otherwise. The external witnesses $w_i$ are the frog PODs themselves that $F$ checks.

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
    - other fun entries like a picture of the frog whisperer, description of the POD, type of POD, etc.
    - signature of POD contentID with website’s `verifier_sk`
- Issue the POD to user’s Zupass “FrogWhisperer” folder

# Local Dev

To run the Frog Counter (prover site), cd into the prover-site and then type the following into the CLI:
```
pnpm install
pnpm dev
```
To run the Frog Whisperer Exchange (verifier site), cd into the verifier-site and type the same commands as above.


