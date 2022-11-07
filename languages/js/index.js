const fs = require("fs");
const alpha = require("./alpha");
const beta = require("./beta");

const readData = (filename) => {
  const input = fs.readFileSync(filename, "utf-8");
  return input.split("\n");
};

const APLHA_RESULT = 0;
const BETA_RESULT = 0;

const example = readData("./example.txt");
const signal = readData("./signal.txt");

console.log("Running alpha...");
const test1 = alpha([...example]);
if (test1 === APLHA_RESULT) {
  console.log("Test passed for alpha!");
  const res1 = alpha([...signal]);
  console.log(`Result for alpha is: ${res1}`);
  console.log("Running beta...");
  const test2 = beta([...example]);
  if (test2 === BETA_RESULT) {
    console.log("Test passed for beta!");
    const res2 = alpha([...signal]);
    console.log(`Result for beta is: ${res2}`);
  } else {
    console.log("Beta test failed");
  }
} else {
  console.log("Alpha test failed");
}
