import { PODEntries } from "@pcd/pod";
import { PODData } from "@parcnet-js/podspec";
import frogWhispererImg from "../assets/frogwhisperer.png";
import { Spinner } from "@chakra-ui/react";
import { useEffect, useState } from "react";
import { useEmbeddedZupass } from "../hooks/useEmbeddedZupass";
import { AddToZupass } from "./AddToZupass";

interface issuePODProps {
  numFrogs: bigint;
}

export function IssuePOD({ numFrogs }: issuePODProps) {
  const { z, connected } = useEmbeddedZupass();
  const [podData, setPODData] = useState<PODData>();
  const [loading, setLoading] = useState(true);
  const [semaphoreIDCommitment, setIDCommit] = useState(0n);
  const [error, setError] = useState<unknown>(null);
  const frogThreshold = 3n;
  const enoughFrogs = numFrogs >= frogThreshold;

  async function getID() {
    // const semaphoreIDCommitment = await z.identity.getSemaphoreV3Commitment();
    const semaphoreIDCommitment = await z.identity.getSemaphoreV4Commitment();
    setIDCommit(semaphoreIDCommitment);
    console.log("semaphoreIDCommitment", semaphoreIDCommitment);
  }

  const generatePOD = async () => {
    try {
      // first, fetch user semaphore id
      const currentDateTime = new Date().toLocaleString();

      const frogWhispererEntries: PODEntries = {
        num_frogs: { type: "int", value: numFrogs },
        owner: {
          type: "string",
          value: semaphoreIDCommitment.toString(),
        },
        timestamp_signed: { type: "string", value: currentDateTime },
        zupass_display: { type: "string", value: "collectable" },
        zupass_image_url: {
          type: "string",
          value:
            "https://upload.wikimedia.org/wikipedia/commons/6/64/Zhuangzi.gif",
        },
        zupass_title: { type: "string", value: "frog whisperer" },
        zupass_description: {
          type: "string",
          value: "gotta catch 'em all",
        },
      };

      const frogWhispererPOD = await z.pod.sign(frogWhispererEntries);
      setPODData(frogWhispererPOD);

      setLoading(false);
    } catch (err: unknown) {
      setError(err);
      setLoading(false);
    }
  };

  useEffect(() => {
    getID();

    if (loading && enoughFrogs && semaphoreIDCommitment != 0n) {
      // only generate frog whisperer POD if user has enough frogs
      generatePOD();
    } else {
      return;
    }
  }, [semaphoreIDCommitment]);

  if (!connected) {
    return (
      <div>
        <p className="mb-4">Zupass disconnected, please reload page.</p>
      </div>
    );
  }

  if (loading) {
    return (
      <div>
        <p className="mb-4">Generating POD...</p>
        <Spinner size="xl" />
      </div>
    );
  }

  if (error) {
    return <>Error while fetching user ID: {error}</>;
  }

  // otherwise, POD is ready to render
  return (
    <div className="flex flex-col gap-4 my-4">
      {enoughFrogs ? (
        <div className="flex flex-col gap-4 my-4">
          {/* <div> */}
          {/* <div> */}
          <div>
            <h1 className="text-xl font-bold mb-2">
              ‧₊˚Congratulations, you are now a Frog Whisperer ‧₊˚
            </h1>
          </div>
          <div>
            <img
              src={frogWhispererImg}
              alt="Frog Whisperer Image"
              style={{ width: "300px", height: "auto" }}
            />
            {/* <pre>[POD displayed here]</pre> */}
          </div>
          {podData ? <AddToZupass podData={podData}></AddToZupass> : <></>}
        </div>
      ) : (
        <h1>
          You must have at least {frogThreshold.toString()} frogs to get a
          FrogWhisperer POD!
        </h1>
      )}
    </div>
  );
}
