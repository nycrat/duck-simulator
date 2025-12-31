import Duck from "./objects/duck";
import { DuckSchema } from "./gen/duck_pb";
import { create, toBinary } from "@bufbuild/protobuf";

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
  const duckState = create(DuckSchema, {
    id: parseInt(duck.duckId),
    x: duck.position.x,
    y: duck.position.y,
    z: duck.position.z,
    rotation: duck.rotation.y,
  });

  return toBinary(DuckSchema, duckState);
}
