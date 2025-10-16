import Duck from "./objects/duck";
import Protos from "../protos_pb";

/**
 * TODO
 */
export function joinGameMessage(duck: Duck) {
  return `join_game\n${duck.duckName}\n${duck.variety}\n${duck.color}`;
}

/**
 * TODO
 */
export function voteStartGameMessage() {
  return `vote_start_game`;
}

/**
 * TODO
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
