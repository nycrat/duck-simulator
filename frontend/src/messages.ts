import Duck from "./objects/duck";
import Protos from "../protos_pb";

/**
 * Message sent to backend, indicates this duck has joined the game
 */
export function joinGameMessage(duck: Duck) {
  return `join_game\n${duck.duckName}\n${duck.variety}\n${duck.color}`;
}

/**
 * Message sent to backend, indicates this duck voted to start game
 */
export function voteStartGameMessage() {
  return `vote_start_game`;
}

/**
 * Binary message sent to backend, indicates this duck's current world state
 */
export function binaryUpdateMessage(duck: Duck) {
  const duckState = new Protos.Duck();
  duckState.setId(duck.duckId);
  duckState.setX(duck.position.x);
  duckState.setY(duck.position.y);
  duckState.setZ(duck.position.z);
  duckState.setRotation(duck.rotation.y);

  return duckState.serializeBinary();
}
