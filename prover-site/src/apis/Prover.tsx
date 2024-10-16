import { PODData } from "@parcnet-js/podspec";
import * as p from "@parcnet-js/podspec";
import { ReactNode, useState } from "react";
import { TryIt } from "../components/TryIt";
import { useEmbeddedZupass } from "../hooks/useEmbeddedZupass";
import { parseFrogPOD, frogPOD } from "../utils/parseData";
import { createProof } from "../utils/runSonobe";
// import { testFrog1, testFrog2, testFrog3 } from "../testfrogs";
import { buildPoseidon } from "circomlibjs";
import { Spinner } from "@chakra-ui/react";

type frogTuple = [bigint, frogPOD];

export function Prover(): ReactNode {
  const { z, connected } = useEmbeddedZupass();
  const [ivc_proof, setIvcProof] = useState(new Uint8Array([]));
  const [proving, setProving] = useState(false);

  // async function makeTestFrogs() {
  //   // add test frogs to Collections/FrogCryptoTest folder while we wait for zupass team to migrate FrogCrypto over to Collections

  //   // console.log("Sample entries", sampleEntries);
  //   const signedTestFrog1POD = await z.pod.sign(testFrog1);
  //   const signedTestFrog2POD = await z.pod.sign(testFrog2);
  //   const signedTestFrog3POD = await z.pod.sign(testFrog3);

  //   console.log("signedTestFrog1POD", signedTestFrog1POD);
  //   console.log("signedTestFrog2POD", signedTestFrog2POD);
  //   console.log("signedTestFrog3POD", signedTestFrog3POD);

  //   await z.pod.collection("FrogCryptoTest").insert(signedTestFrog1POD);
  //   await z.pod.collection("FrogCryptoTest").insert(signedTestFrog2POD);
  //   await z.pod.collection("FrogCryptoTest").insert(signedTestFrog3POD);

  //   console.log(
  //     "should have added testFrog1, testFrog2, testFrog3 to folder Collections/FrogCryptoTest"
  //   );

  //   return;
  // }

  async function computePoseidonHash(inputArray: Array<bigint>) {
    const poseidon = await buildPoseidon();
    const hash = poseidon.F.toString(poseidon(inputArray));
    return hash;
  }

  async function getFrogHashInput(parsedFrog: frogPOD) {
    return [
      BigInt(parsedFrog.frogId),
      BigInt(parsedFrog.biome),
      BigInt(parsedFrog.rarity),
      BigInt(parsedFrog.temperament),
      BigInt(parsedFrog.jump),
      BigInt(parsedFrog.speed),
      BigInt(parsedFrog.intelligence),
      BigInt(parsedFrog.beauty),
      BigInt(parsedFrog.timestampSigned),
      BigInt(parsedFrog.ownerSemaphoreId),
      BigInt(parsedFrog.reservedField1),
      BigInt(parsedFrog.reservedField2),
      BigInt(parsedFrog.reservedField3),
    ];
  }

  async function getID() {
    const semaphoreIDCommitment = await z.identity.getSemaphoreV3Commitment();
    return semaphoreIDCommitment;
  }

  function createSortedFrogJson(frogArray: frogPOD[]): {
    [key: number]: frogPOD;
  } {
    return frogArray.reduce<{ [key: number]: frogPOD }>(
      (accumulator, currentValue, index) => {
        accumulator[index + 1] = currentValue;
        return accumulator;
      },
      {}
    );
  }

  async function processProofInputs(): Promise<object> {
    // first make test frogs to insert to FrogCryptoTest
    // await makeTestFrogs(); // only run once
    // console.log("made test frogs!");

    // then get semaphore ID commitment
    const semaphoreIDCommitment = await getID();
    console.log("semaphoreIDCommitment", semaphoreIDCommitment);
    // const semaphoreIDCommitment = result;

    const frogPODList = await z.pod.collection("FrogCryptoTest").query(
      // lookin for them frogs
      p.pod({
        entries: {
          frogId: { type: "string" },
        },
      })
    );

    // console.log("frogPCDList", frogPODList);

    const promises = frogPODList.map(async (frogPOD: PODData) => {
      // console.log("frogPOD", frogPOD);
      const result = await parseFrogPOD(frogPOD, semaphoreIDCommitment);
      // console.log("parsedFrogPOD: ", result);
      return result;
    });

    const parsedFrogs = await Promise.all(promises);

    console.log("parsedFrogs", parsedFrogs);

    // sort frogs now
    // create an array of [frogMsgHash, parsedfrog]
    const frogHash_and_frog_array: Array<frogTuple> = await Promise.all(
      parsedFrogs.map(async (parsedFrog: frogPOD) => {
        const frogHashInput = await getFrogHashInput(parsedFrog);
        const frogMsgHash = await computePoseidonHash(frogHashInput);
        return [frogMsgHash, parsedFrog];
      })
    );

    console.log("initial frogHash_and_frog_array", frogHash_and_frog_array);
    // sort frogs by message hash
    frogHash_and_frog_array.sort((a: frogTuple, b: frogTuple) =>
      Number(a[0] - b[0])
    );
    console.log("sorted frogHash_and_frog_array", frogHash_and_frog_array);
    // remove the message hashes
    const sortedFrogArray = frogHash_and_frog_array.map(
      (item: frogTuple) => item[1]
    );

    const sortedFrogJson = createSortedFrogJson(sortedFrogArray);

    console.log("sortedFrogJson", sortedFrogJson);

    return sortedFrogJson;
  }

  async function prove() {
    setProving(true);
    const circuitInputs = await processProofInputs();
    const ivc_proof = await createProof(circuitInputs);
    if (ivc_proof) {
      setIvcProof(ivc_proof);
      console.log("ivc_proof", ivc_proof);
    } else {
      console.log("ivc_proof was undefined");
    }
  }

  const downloadBinFile = (uint8Array: Uint8Array, fileName: string) => {
    // Create a Blob from the Uint8Array
    const blob = new Blob([uint8Array], { type: "application/octet-stream" });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = fileName;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  };

  return !connected ? null : (
    <div className="flex flex-col gap-4 my-4">
      <h1 className="text-xl font-bold mb-2">Count your frogs</h1>
      <p>You are now connected to your Zupass account!</p>
      <p>
        Press the button below to generate a ZK Proof about how many frogs you
        have in your FrogCrypto folder.
      </p>

      <div>
        <TryIt onClick={prove} label="Fetch inputs and prove" />
      </div>

      <div>
        {!proving ? null : (
          <>
            {ivc_proof.length > 0 ? (
              <>
                <p className="whitespace-pre-wrap">
                  {"Created proof! ◝(ᵔᵕᵔ)◜"}
                </p>
                <div className="flex flex-col gap-4 my-4">
                  <h1 className="text-xl font-bold mb-2">Download proof</h1>
                  <p>
                    {
                      "Now press the button below to download your proof file, which you can upload and verify "
                    }
                    <a
                      href="https://frog-whisperer-exchange-jmsi17vt5-janabels-projects.vercel.app/"
                      style={{ color: "green", textDecoration: "underline" }}
                    >
                      here
                    </a>
                    .
                  </p>

                  <div>
                    <TryIt
                      onClick={() =>
                        downloadBinFile(ivc_proof, "ivc_proof.bin")
                      }
                      label="Download proof"
                    />
                  </div>
                </div>
              </>
            ) : (
              <div>
                <p className="mb-4">Proving...</p>
                <Spinner size="xl" />
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
}
