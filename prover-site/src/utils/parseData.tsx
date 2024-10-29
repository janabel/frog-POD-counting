import { PODData } from "@parcnet-js/podspec";
import { POD } from "@pcd/pod";
import { PODName, PODStringValue, PODValue } from "@pcd/pod";
import { decodeSignature } from "@pcd/pod";
import { SIGNATURE_REGEX, SIGNATURE_ENCODING_GROUPS } from "@pcd/pod";
import { decodePublicKey } from "@pcd/pod";
import { PUBLIC_KEY_REGEX, PUBLIC_KEY_ENCODING_GROUPS } from "@pcd/pod";
import { podMerkleTreeHash, podNameHash, podValueHash } from "@pcd/pod";
import { LeanIMT, LeanIMTMerkleProof } from "@zk-kit/lean-imt";

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
  contentID: string;
}

export async function getContentID(frogPODData: PODData) {
  const entries = frogPODData.entries;

  // create new POD from PODdata
  const frogPOD = POD.load(
    frogPODData.entries,
    frogPODData.signature,
    frogPODData.signerPublicKey
  );
  const contentID = frogPOD.contentID;
  return contentID;
}

export async function parseFrogPOD(
  frogPODData: PODData,
  semaphoreIdentityCommitment: bigint
): Promise<frogPOD> {
  const entries = frogPODData.entries;
  console.log("entries", entries);

  const signature = frogPODData.signature;
  const signerPublicKey = frogPODData.signerPublicKey;
  const contentID = await getContentID(frogPODData);
  console.log("contentID", contentID);

  const unpackedSignature = decodeSignature(signature);
  console.log("unpackedSignature", unpackedSignature);
  const frogSignatureR8x = unpackedSignature.R8[0];
  const frogSignatureR8y = unpackedSignature.R8[1];
  const frogSignatureS = unpackedSignature.S;

  const unpackedPublicKey = decodePublicKey(signerPublicKey);
  console.log("unpackedSignature", unpackedPublicKey);
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
    contentID: contentID.toString(),
  };
}
