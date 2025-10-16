import * as THREE from "three";
import { GLTF, GLTFLoader } from "three/examples/jsm/Addons.js";
import { Text } from "troika-three-text";

/**
 * Variety of duck (skins)
 */
export enum DuckVariety {
  DUCK,
  RABBIT,
}

let duck_glbs: Map<DuckVariety, GLTF> = new Map();

/**
 * A duck/player
 *
 * @example
 * ```typescript
 * const duck = new Duck("Ducky", DuckVariety.RABBIT, color: "#ffff00")
 * ```
 */
export default class Duck extends THREE.Group {
  direction: number = Math.PI;
  deltaDirection: number = 0;
  size: THREE.Vector3;
  duckId: string = "";
  nameText: Text;
  duckName: string;
  score: number = 0;
  variety: DuckVariety;
  color: string;

  constructor(duckName: string, variety: DuckVariety, color: string) {
    super();

    this.variety = variety;
    this.color = color;

    const loader = new GLTFLoader();
    loader.load(
      `${DuckVariety[variety].toLowerCase()}.glb`,
      (glb) => {
        glb.scene.castShadow = true;
        glb.scene.name = "duck";

        glb.scene.traverse(function (child) {
          child.castShadow = true;
          child.receiveShadow = true;
        });

        duck_glbs.set(variety, glb);
        this.add(glb.scene.clone());
        this.updateColor(color);
      },
      undefined,
      (err) => {
        console.error(err);
      },
    );

    this.duckName = duckName;
    this.position.y = -0.1;

    this.nameText = new Text();
    this.add(this.nameText);

    this.nameText.text = duckName + "\n0";
    this.nameText.textAlign = "center";
    this.nameText.fontSize = 0.2;
    this.nameText.anchorX = "center";
    this.nameText.position.y = 2;
    this.nameText.color = 0xffffff;

    this.nameText.sync();

    this.size = new THREE.Vector3(1, 1, 1);
    this.size.multiplyScalar(0.5);

    this.nameText.visible = false;
    this.rotation.set(0, this.direction, 0);
  }

  /**
   * Moves duck forward and rotates duck
   */
  update(deltaTime: number) {
    this.direction += this.deltaDirection * deltaTime;

    const deltaPos = new THREE.Vector3(
      Math.sin(this.direction),
      0,
      Math.cos(this.direction),
    );
    deltaPos.multiplyScalar(deltaTime * 3);

    this.position.add(deltaPos);
    this.rotation.set(0, this.direction, 0);
  }

  /**
   * Changes the displayed score on duck's text
   */
  updateScore() {
    this.nameText.text = this.duckName + "\n" + this.score;
  }

  /**
   * Changes the color of duck
   */
  updateColor(color: string) {
    const duck_model = this.getObjectByName("duck")!;
    this.color = color;

    if (color === "#000000") {
      return;
    }
    duck_model.traverse(function (child) {
      // @ts-ignore
      if (child.isMesh) {
        // @ts-ignore
        const duckColor = child.material.color;
        if (duckColor.r !== 0 || duckColor.g !== 0 || duckColor.b !== 0) {
          duckColor.set(color);
        }
      }
    });
  }
}
