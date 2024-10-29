# Frog Counting

Frog Counting will consist of two websites built on Zupass: Frog Counter and Frog Whisperer.

Frog Whisperer (built on Zupass) will ultimately issue the user `FrogWhisperer` PODs, i.e. PODs that certify ownership of many Frog PODs from [FrogCrypto][https://frogcrypto.vercel.app/].

The user first navigates to Frog Counter. Frog Counter will allow the user to i) fetch their Frogs from Zupass and ii) generate a proof that says “I have N frogs” (where N can be very large, thanks to Sonobe/folding schemes). The website then:

- Gets an array of `Frogs` from Zupass (using the new ZApp API!)
- Generate Circuit Inputs
  - Create `Frogs` (automatically passed into circuit), which have entries:
    - `ownerSemaphoreID` (user’s Semaphore ID)
    - `frogSignerPubkey` of frog POD issuer
    - `frogSignature` from frog POD issuer, signs (merkle root of) frog POD contents
    - `semaphoreIdentityTrapdoor`, `semaphoreIdentityNullifier` fetched from user's identity PCD
    - ...
  - Hardcodes (not useful) `watermark=2718`, `reservedField{i}=0`, `externalNullifier=STATIC_ZK_EDDSA_FROG_PCD_NULLIFIER`
- Generates ZK proof with the circuit inputs above (using the Sonobe library for folding schemes)
- Outputs the Sonobe `proof`, which has entries:
  - `public_inputs` (= public outputs), includes N = number of frogs) [?]
  - `vkey` = `nova_cyclefold_vk`
  - `proof` = `D::prove(rng, decider_pp, nova.clone()).unwrap()`

The user will then navigate back to the original Frog Rewards website and paste in the `proof`, `public_inputs`, and `vkey`. Then the website will:

- Verify the proof in the backend
- Issue a `FrogWhisperer` POD signed with the website’s secret key `verifier_sk`
  - POD contains:
    - PODEntries:
      - `public_signals` = # of frogs that user owns
      - picture of frog whisperer :), description, title (flavortext)
      - owner = `semaphoreID.commitment` (user’s public semaphore ID)
    - signature of POD content (PODEntries) with website’s `verifier_sk`
- Put the POD in the user’s Zupass “FrogWhisperer” folder

# Local Dev
Run prover site with
```
cd prover-site
pnpm install
pnpm dev
```
