import * as THREE from "three";
import { Water } from "three/examples/jsm/Addons.js";

export default class Pond extends Water {
  constructor(radius: number) {
    super(new THREE.CircleGeometry(radius, 32), {
      textureWidth: 2048,
      textureHeight: 2048,
      waterNormals: new THREE.TextureLoader().load(
        "waternormals.jpg",
        function (texture) {
          texture.wrapS = texture.wrapT = THREE.RepeatWrapping;
        },
      ),
      sunDirection: new THREE.Vector3(),
      sunColor: 0xffe099,
      waterColor: 0x050e30,
      distortionScale: 0.3,
      fog: false,
    });
    this.name = "pond";

    this.rotation.x = -Math.PI / 2;
    this.material.uniforms["size"].value = 1;
  }

  update(deltaTime: number) {
    this.material.uniforms["time"].value += deltaTime * 0.2;
  }
}
