// import { POD } from "@pcd/pod";
import { PODData } from "@parcnet-js/podspec";

import { ReactNode, useMemo, useState } from "react";
import { useEmbeddedZupass } from "../hooks/useEmbeddedZupass";
import { ZUPASS_URL } from "../constants";

export interface AddToZupassProps {
  podData: PODData;
}

export function AddToZupass({ podData }: AddToZupassProps): ReactNode {
  const { z, connected } = useEmbeddedZupass();
  // const [list, setList] = useState<ZupassFolderContent[]>([]);
  // const [pcd, setPCD] = useState<SerializedPCD>();
  const [pcdAdded, setPCDAdded] = useState(false);
  const zupassUrl = useMemo(() => {
    return localStorage.getItem("zupassUrl") || ZUPASS_URL;
  }, []);

  console.log("frogwhisperer pod", podData);

  return !connected ? null : (
    <div className="flex flex-col gap-4 my-4">
      {/* // <div> */}
      {/* <div> */}
      <h1 className="text-xl font-bold mb-2">
        + Add your Frog Whisperer POD to Zupass +
      </h1>

      <div>
        <button
          className="btn btn-primary"
          onClick={async () => {
            if (pcdAdded) {
              // if already added, exit
              alert("You can only add this POD once!");
              return;
            }

            try {
              setPCDAdded(true);
              await z.pod.collection("FrogWhisperer").insert(podData);
              console.log("added frog pod!");
            } catch (e) {
              console.log(e);
            }
          }}
        >
          Add POD
        </button>
      </div>

      {/* </div> */}
      {pcdAdded && (
        <p>
          PCD added!{" "}
          <a
            href={zupassUrl}
            target="_blank"
            style={{ color: "green", textDecoration: "underline" }}
          >
            View in Zupass
          </a>
          .
        </p>
      )}
    </div>
  );
}
