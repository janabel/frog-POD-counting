import { EmbeddedZupassProvider } from "./hooks/useEmbeddedZupass";
import { Navbar } from "./components/Navbar";
import { Prover } from "./apis/Prover";
import { ZUPASS_URL } from "./constants";
import { ChakraProvider } from "@chakra-ui/react";
import { Header } from "./components/Header";
import { Zapp } from "@parcnet-js/app-connector";

const zapp: Zapp = {
  name: "Frog Counter",
  permissions: {
    REQUEST_PROOF: {
      collections: ["FrogCryptoTest", "FrogWhisperer", "FrogCrypto", "/"],
    },
    SIGN_POD: {},
    READ_POD: {
      collections: ["FrogCryptoTest", "FrogWhisperer", "FrogCrypto", "/"],
    },
    INSERT_POD: {
      collections: ["FrogCryptoTest", "FrogWhisperer", "FrogCrypto", "/"],
    },
    DELETE_POD: {
      collections: ["FrogCryptoTest", "FrogWhisperer", "FrogCrypto", "/"],
    },
    READ_PUBLIC_IDENTIFIERS: {},
  },
};

console.log("testing zapp setup", zapp);

function App() {
  const zupassUrl = localStorage.getItem("zupassUrl") || ZUPASS_URL;

  return (
    <ChakraProvider>
      <EmbeddedZupassProvider zapp={zapp} zupassUrl={zupassUrl}>
        <Navbar />
        <div className="container mx-auto my-4 p-4">
          <Header />
          <Prover />
        </div>
      </EmbeddedZupassProvider>
    </ChakraProvider>
  );
}

export default App;
