import init, { hello } from "../pkg/kinematics_wasm.js";

async function main() {
  await init();
  const message = hello();
  console.log(message);
  document.getElementById("app")!.textContent = message;
}

main();
