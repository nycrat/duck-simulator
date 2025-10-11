import * as THREE from "three";
import { GLTF, GLTFLoader } from "three/examples/jsm/Addons.js";

let bread_glb: GLTF;

export default class Bread extends THREE.Group {
  velocityY: number;
  size: THREE.Vector3;

  constructor(x?: number, y?: number, z?: number) {
    super();
    if (bread_glb) {
      this.add(bread_glb.scene.clone());
    } else {
      const loader = new GLTFLoader();
      loader.load(
        "bred.glb",
        (glb) => {
          glb.scene.castShadow = true;
          glb.scene.traverse(function (child) {
            child.castShadow = true;
            child.receiveShadow = true;
          });
          bread_glb = glb;
          this.add(bread_glb.scene.clone());
        },
        undefined,
        (err) => {
          console.error(err);
        },
      );
    }

    this.velocityY = 0;
    this.size = new THREE.Vector3(0.2, 0.2, 0.4);
    this.size.multiplyScalar(0.5);

    if (x && y && z) {
      this.position.set(x, y, z);
    } else {
      this.position.setX(Math.random() * 10 - 5);
      this.position.setZ(Math.random() * 10 - 5);
      this.position.setY(Math.random() * 10 + 5);
    }
  }

  update(deltaTime: number) {
    if (this.position.y <= 0) {
      this.position.setY(0);
      this.velocityY = 0;
      return;
    }

    const GRAVITY = -5;
    this.velocityY += GRAVITY * deltaTime;
    this.position.setY(this.position.y + this.velocityY * deltaTime);
  }
}
