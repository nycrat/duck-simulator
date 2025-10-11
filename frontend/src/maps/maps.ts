import Game from "../game";
import initCloudsMap from "./clouds";
import initDefaultMap from "./default";
import initOceanMap from "./ocean";

export enum GameMap {
  DEFAULT,
  CLOUDS,
  OCEAN,
}

const mapInitFunctions: Map<GameMap, (game: Game) => void> = new Map();

[
  { map: GameMap.DEFAULT, func: initDefaultMap },
  { map: GameMap.CLOUDS, func: initCloudsMap },
  { map: GameMap.OCEAN, func: initOceanMap },
].forEach((a) => {
  mapInitFunctions.set(a.map, a.func);
});

export default function initializeMap(map: GameMap, game: Game) {
  mapInitFunctions.get(map)!(game);
}
