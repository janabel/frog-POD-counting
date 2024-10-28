import { PODData } from "@parcnet-js/podspec";
import { PODName, PODStringValue } from "@pcd/pod";
import { decodeSignature } from "@pcd/pod";
import { SIGNATURE_REGEX, SIGNATURE_ENCODING_GROUPS } from "@pcd/pod";
import { decodePublicKey } from "@pcd/pod";
import { PUBLIC_KEY_REGEX, PUBLIC_KEY_ENCODING_GROUPS } from "@pcd/pod";

interface Owner {
  cryptographic: string;
  type: "string";
  value: string;
}

export interface frogPOD {
  frogId: string;
  biome: string;
  rarity: string;
  temperament: string;
  jump: string;
  speed: string;
  intelligence: string;
  beauty: string;
  timestampSigned: string;
  ownerSemaphoreId: string;
  frogSignerPubkeyAx: string;
  frogSignerPubkeyAy: string;
  semaphoreIdentityCommitment: string;
  watermark: string;
  frogSignatureR8x: string;
  frogSignatureR8y: string;
  frogSignatureS: string;
  externalNullifier: string;
  reservedField1: string;
  reservedField2: string;
  reservedField3: string;
}

export async function parseFrogPOD(
  frogPOD: PODData,
  semaphoreIdentityCommitment: bigint
): Promise<frogPOD> {
  const entries = frogPOD.entries;

  const signature = frogPOD.signature;
  const signerPublicKey = frogPOD.signerPublicKey;

  const unpackedSignature = decodeSignature(signature);
  // console.log('unpackedSignature', unpackedSignature);
  const frogSignatureR8x = unpackedSignature.R8[0];
  const frogSignatureR8y = unpackedSignature.R8[1];
  const frogSignatureS = unpackedSignature.S;

  const unpackedPublicKey = decodePublicKey(signerPublicKey);
  const frogSignerPubkeyAx = unpackedPublicKey[0];
  const frogSignerPubkeyAy = unpackedPublicKey[1];

  console.log("frogSignatureR8x", frogSignatureR8x);
  console.log("frogSignatureR8y", frogSignatureR8y);
  console.log("frogSignatureS", frogSignatureS);
  console.log("frogSignerPubkeyAx", frogSignerPubkeyAx);
  console.log("frogSignerPubkeyAy", frogSignerPubkeyAy);

  return {
    frogId: entries.frogId.value.toString(),
    biome: entries.biome.value.toString(),
    rarity: entries.rarity.value.toString(),
    temperament: entries.temperament.value.toString(),
    jump: entries.jump.value.toString(),
    speed: entries.speed.value.toString(),
    intelligence: entries.intelligence.value.toString(),
    beauty: entries.beauty.value.toString(),
    timestampSigned: entries.timestampSigned.value.toString(),
    // ownerSemaphoreId: (entries.owner as Owner).cryptographic,
    ownerSemaphoreId: entries.owner.value.toString(),
    frogSignerPubkeyAx: frogSignerPubkeyAx.toString(),
    frogSignerPubkeyAy: frogSignerPubkeyAy.toString(),
    semaphoreIdentityCommitment: semaphoreIdentityCommitment.toString(),
    watermark: "2178",
    frogSignatureR8x: frogSignatureR8x.toString(),
    frogSignatureR8y: frogSignatureR8y.toString(),
    frogSignatureS: frogSignatureS.toString(),
    externalNullifier:
      "10661416524110617647338817740993999665252234336167220367090184441007783393", // dummy because we don't care about this nullifier / uniqueness of frogs is already taken care of by sorting
    reservedField1: "0",
    reservedField2: "0",
    reservedField3: "0",
  };
}

// // use while waiting for zupass
// export async function parseFrogPODTemp(
//   frogPOD: PODData,
//   semaphoreIdentityCommitment: bigint
// ): Promise<frogPOD> {
//   // dummy test pods have all entries already formatted correctly

//   const entries = frogPOD.entries;

//   return {
//     frogId: entries.frogId.value.toString(),
//     biome: entries.biome.value.toString(),
//     rarity: entries.rarity.value.toString(),
//     temperament: entries.temperament.value.toString(),
//     jump: entries.jump.value.toString(),
//     speed: entries.speed.value.toString(),
//     intelligence: entries.intelligence.value.toString(),
//     beauty: entries.beauty.value.toString(),
//     timestampSigned: entries.timestampSigned.value.toString(),
//     ownerSemaphoreId: entries.ownerSemaphoreId.value.toString(),
//     frogSignerPubkeyAx: entries.frogSignerPubkeyAx.value.toString(),
//     frogSignerPubkeyAy: entries.frogSignerPubkeyAy.value.toString(),
//     semaphoreIdentityCommitment: semaphoreIdentityCommitment.toString(),
//     watermark: entries.watermark.value.cryptographic.toString(),
//     frogSignatureR8x: entries.frogSignatureR8x.value.toString(),
//     frogSignatureR8y: entries.frogSignatureR8y.value.toString(),
//     frogSignatureS: entries.frogSignatureS.value.toString(),
//     externalNullifier: entries.externalNullifier.value.toString(),
//     reservedField1: entries.reservedField1.value.toString(),
//     reservedField2: entries.reservedField2.value.toString(),
//     reservedField3: entries.reservedField3.value.toString(),
//   };
// }

// type Owner = {
//   cryptographic: string;
//   value: object;
// };
// interface StringEntry {
//   type: "string";
//   value: string;
// }
// export type PODValue = StringEntry | Owner;
// type PODEntries = Record<PODName, PODValue>;

// export interface PODData {
//   entries: PODEntries;
//   signature: string;
//   signerPublicKey: string;
// }

// export type StringPOD = {
//   entries: Record<PODName, PODValue>;
//   signature: string;
//   signerPublicKey: string;
// };

// function assert(condition: boolean, message = "Assertion failed") {
//   console.log(condition);
//   if (!condition) {
//     throw new Error(message);
//   }
// }

// function base64ToHex(base64String: string): string {
//   const binaryString = atob(base64String);
//   let hexString = "";
//   for (let i = 0; i < binaryString.length; i++) {
//     const hex = binaryString.charCodeAt(i).toString(16).padStart(2, "0");
//     hexString += hex;
//   }
//   return hexString;
// }

// function extractSignerPublicKeyCoordinates(base64PubKey: string): {
//   frogSignerPubkeyAx: string;
//   frogSignerPubkeyAy: string;
// } {
//   // Step 1: Decode the Base64 public key
//   const binaryPubKey = Uint8Array.from(atob(base64PubKey), (c) =>
//     c.charCodeAt(0)
//   );

//   // Step 2: Check the length of the decoded public key
//   if (binaryPubKey.length !== 32) {
//     throw new Error("Invalid public key length. Expected 32 bytes.");
//   }

//   // Step 3: Extract x and y coordinates
//   const frogSignerPubkeyAx = Array.from(binaryPubKey.slice(0, 16))
//     .map((byte) => byte.toString(16).padStart(2, "0"))
//     .join(""); // First 16 bytes (128 bits) for x
//   const frogSignerPubkeyAy = Array.from(binaryPubKey.slice(16, 32))
//     .map((byte) => byte.toString(16).padStart(2, "0"))
//     .join(""); // Last 16 bytes (128 bits) for y

//   return {
//     frogSignerPubkeyAx,
//     frogSignerPubkeyAy,
//   };
// }
