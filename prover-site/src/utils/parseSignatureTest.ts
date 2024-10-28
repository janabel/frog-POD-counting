function padSignature(signatureBase64: string): string {
  if (signatureBase64.length % 3 == 1) {
    return signatureBase64 + "==";
  } else if (signature.length % 3 == 2) {
    return signatureBase64 + "=";
  } else {
    return signatureBase64;
  }
}

function parseSignature(signatureBase64: string) {
  const paddedSignatureBase64 = padSignature(signatureBase64);
  // Decode the Base64 signature
  const signatureBytes = Buffer.from(paddedSignatureBase64, "base64");

  // Check if we have 64 bytes for EdDSA/Ed25519 standard format
  if (signatureBytes.length !== 64) {
    throw new Error("Invalid signature length. Expected 64 bytes.");
  }

  // Split signature bytes
  const R8x = signatureBytes.slice(0, 16).toString("hex");
  const R8y = signatureBytes.slice(16, 32).toString("hex");
  const S = signatureBytes.slice(32, 64).toString("hex");

  return {
    frogSignatureR8x: R8x,
    frogSignatureR8y: R8y,
    frogSignatureS: S,
  };
}

// Example usage
const signature =
  "Re2yb2nlSZwxdKR0kEIZQIcRFm7hGph7/518LIv/Pp0Q5NU7tw1hdfG3K/uZvZD130I10FgyOQqUJts901v8AQ";
const parsedSignature = parseSignature(signature);
console.log(parsedSignature);
