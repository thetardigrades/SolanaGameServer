module.exports = {
  clearMocks: true,
  testEnvironment: "node",
  testMatch: ["<rootDir>/src/tests/*.(ts|js|jsx?|tsx?)"],
  transform: {
    "^.+\\.(t|j)sx?$": "ts-jest",
  },
  moduleFileExtensions: ["ts", "tsx", "js", "jsx", "json", "node"],
  // Jest screws with the definition of Buffer to not be a UInt8Array which breaks Solana:
  // Error: https://gist.github.com/gilgameshcoder/17edc642945d1db9b3a4a00dc79515b2
  // Fix (copied below): https://github.com/facebook/jest/issues/4422
  globals: {
    Uint8Array: Uint8Array,
    ArrayBuffer: ArrayBuffer,
  },
};
