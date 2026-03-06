import { RequestMessage } from "./types";
// @ts-ignore
import FBWorker from "./webworker?worker";

const fbWorker = new FBWorker();
console.log("Worker initialized:", fbWorker);

// This ensures all messages are correctly typed
export function postToWorker(message: RequestMessage) {
  fbWorker.postMessage(message);
}

export default fbWorker;
