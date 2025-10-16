import { RGBELoader } from "three/examples/jsm/Addons.js";
import Game from "../game";
import * as THREE from "three";
import Pond from "../objects/pond";

export default function initCloudsMap(game: Game) {
  new RGBELoader().load("sky.hdr", (texture) => {
    texture.mapping = THREE.EquirectangularReflectionMapping;
    game.scene.background = texture;
    game.scene.environment = texture;
  });

  const pond = new Pond(10000);

  game.scene.add(pond);
}
