import * as THREE from "three";
import Duck, { DuckVariety } from "./objects/duck";
import Bread from "./objects/bread";
import { GameMode, POV } from "./options";
import Stats from "three/examples/jsm/libs/stats.module.js";
import Pond from "./objects/pond";
import { FirstPersonControls } from "three/examples/jsm/Addons.js";

/**
 * Singleton game instance
 */
export default class Game {
  clock: THREE.Clock;
  pressedKeys: Map<string, boolean>;
  scene: THREE.Scene;
  camera: THREE.PerspectiveCamera;
  renderer: THREE.WebGLRenderer;
  ducks: Duck[];
  breadList: Bread[] = [];
  pov: POV = POV.THIRD_PERSON;
  gameMode: GameMode = GameMode.MENU;
  stats: Stats = new Stats();
  mapSize: number = 100000;
  mapCircular: boolean = true;
  startTime: number = 0;

  showStats = false;
  controls: FirstPersonControls;
  gameDuration = 0;

  constructor() {
    this.clock = new THREE.Clock();
    this.pressedKeys = new Map();
    this.scene = new THREE.Scene();
    this.camera = new THREE.PerspectiveCamera(
      90,
      window.innerWidth / window.innerHeight,
      0.1,
      1000,
    );
    this.renderer = new THREE.WebGLRenderer({
      antialias: true,
      precision: "highp",
      powerPreference: "high-performance",
    });

    this.renderer.shadowMap.enabled = true;
    this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;
    this.renderer.toneMapping = THREE.ACESFilmicToneMapping;

    document.body.appendChild(this.renderer.domElement);

    this.ducks = [new Duck("Ducky", DuckVariety.DUCK, "#f1f566")];
    this.ducks[0].updateScore();
    this.scene.add(this.ducks[0]);

    this.stats.showPanel(3);
    document.body.appendChild(this.stats.dom);

    this.controls = new FirstPersonControls(
      this.camera,
      document.getElementById("ha")!,
    );
    this.controls.movementSpeed = 2;
    this.controls.lookSpeed = 0.2;
    // this.ontrols.rollSpeed = 1;
  }

  /**
   * Add event listeners for key press and touch, add and removes keys to `pressedKeys`
   */
  initControls() {
    const self = this;

    const handleTouch = (touch: Touch) => {
      const touchX = touch.clientX;
      if (touchX > window.innerWidth / 2) {
        self.pressedKeys.set("ArrowRight", true);
        self.pressedKeys.set("ArrowLeft", false);
      } else {
        self.pressedKeys.set("ArrowLeft", true);
        self.pressedKeys.set("ArrowRight", false);
      }
    };

    window.addEventListener("keydown", (event) => {
      self.pressedKeys.set(event.key, true);
    });
    window.addEventListener("keyup", (event) => {
      self.pressedKeys.set(event.key, false);
    });
    window.addEventListener(
      "touchstart",
      (event) => {
        event.preventDefault();
        handleTouch(event.touches[event.touches.length - 1]);
      },
      { passive: false },
    );
    window.addEventListener("touchend", (event) => {
      self.pressedKeys.set("ArrowRight", false);
      self.pressedKeys.set("ArrowLeft", false);

      if (event.touches.length > 0) {
        handleTouch(event.touches[event.touches.length - 1]);
      }
    });
  }

  /**
   * Renders 3D game world
   */
  render(self: Game) {
    requestAnimationFrame(() => self.render(self));
    self.renderer.render(self.scene, self.camera);
    this.stats.update();
  }

  /**
   * Updates game world to next tick
   */
  update() {
    const self = this;
    const deltaTime = self.clock.getDelta();

    // TODO UPDAT ETHIS
    if (self.gameMode === GameMode.SPECTATOR) {
      self.ducks[0].visible = false;
    }

    const a = new THREE.Vector2();
    this.renderer.getSize(a);
    if (window.innerWidth !== a.width || window.innerHeight !== a.height) {
      self.camera.aspect = window.innerWidth / window.innerHeight;
      self.camera.updateProjectionMatrix();

      self.renderer.setSize(window.innerWidth, window.innerHeight);
      self.renderer.setPixelRatio(window.devicePixelRatio);
    }

    const pond = self.scene.getObjectByName("pond");
    (<Pond>pond).update(deltaTime);

    // UPDATE TIME
    if (
      self.gameMode === GameMode.ONLINE ||
      self.gameMode === GameMode.SPECTATOR
    ) {
      const elapsedTime = new Date().getTime() / 1000 - self.startTime;
      const curTime = Math.trunc(this.gameDuration - elapsedTime);

      // @ts-ignore
      const zeroPad = (num, places) => String(num).padStart(places, "0");

      document.getElementById("timer")!.innerText = `${zeroPad(
        Math.trunc(curTime / 60),
        2,
      )}:${zeroPad(curTime % 60, 2)}`;

      document.getElementById("timer")!.innerText +=
        self.gameMode === GameMode.SPECTATOR ? " (Spectating)" : "";
    }

    if (
      self.gameMode === GameMode.MENU ||
      self.gameMode === GameMode.LEADERBOARDS
    ) {
      for (const duck of self.ducks) {
        duck.nameText.lookAt(self.camera.position);
      }
      return;
    }

    if (Math.random() <= 5 * deltaTime && self.gameMode === GameMode.OFFLINE) {
      if (self.mapCircular) {
        const theta = Math.random() * Math.PI * 2;
        const r = Math.random() * self.mapSize;
        self.breadList.push(
          new Bread(
            Math.sin(theta) * r,
            Math.random() * 10 + 5,
            Math.cos(theta) * r,
          ),
        );
        self.scene.add(self.breadList[self.breadList.length - 1]);
      } else {
        self.breadList.push(new Bread());
        self.scene.add(self.breadList[self.breadList.length - 1]);
      }
    }

    for (let i = 0; i < self.breadList.length; i++) {
      self.breadList[i].update(deltaTime);
    }

    for (const duck of self.ducks) {
      duck.update(deltaTime);
      duck.nameText.lookAt(self.camera.position);
      duck.updateScore();

      if (self.gameMode !== GameMode.ZEN) {
        if (self.mapCircular) {
          const x = duck.position.x;
          const z = duck.position.z;
          const c = Math.sqrt(x * x + z * z);
          if (c > self.mapSize) {
            const ratio = self.mapSize / c;
            duck.position.x = x * ratio;
            duck.position.z = z * ratio;
          }
        } else {
          if (Math.abs(duck.position.x) > self.mapSize) {
            duck.position.setX(self.mapSize * Math.sign(duck.position.x));
          }
          if (Math.abs(duck.position.z) > self.mapSize) {
            duck.position.setZ(self.mapSize * Math.sign(duck.position.z));
          }
        }
      }
    }

    self.handleCollisions();

    if (self.gameMode === GameMode.SPECTATOR) {
      this.controls.update(deltaTime);
      if (self.camera.position.y < 0.2) {
        self.camera.position.y = 0.2;
      }
    } else {
      self.handleInput();
      self.updateCamera();
    }
  }

  /**
   * Updates camera based on game mode and POV
   */
  updateCamera() {
    const self = this;

    if (self.gameMode === GameMode.LEADERBOARDS) {
      self.camera.position.set(0, 1, 3);
      self.camera.lookAt(new THREE.Vector3(0, 1, -1));
    } else if (self.pov === POV.FIRST_PERSON) {
      self.camera.position.set(0, 1, 0);
      self.camera.position.add(self.ducks[0].position);

      const lookat = new THREE.Vector3(
        self.ducks[0].position.x + 200 * Math.sin(self.ducks[0].direction),
        self.ducks[0].position.y,
        self.ducks[0].position.z + 200 * Math.cos(self.ducks[0].direction),
      );
      self.camera.lookAt(lookat);
    } else if (self.pov === POV.SECOND_PERSON) {
      self.camera.position.set(
        Math.sin(self.ducks[0].direction) * 5,
        4,
        Math.cos(self.ducks[0].direction) * 5,
      );
      self.camera.position.add(self.ducks[0].position);

      const lookat = new THREE.Vector3(
        self.ducks[0].position.x - 2 * Math.sin(self.ducks[0].direction),
        self.ducks[0].position.y,
        self.ducks[0].position.z - 2 * Math.cos(self.ducks[0].direction),
      );
      self.camera.lookAt(lookat);
    } else if (self.pov === POV.THIRD_PERSON) {
      self.camera.position.set(
        -Math.sin(self.ducks[0].direction) * 5,
        4,
        -Math.cos(self.ducks[0].direction) * 5,
      );
      self.camera.position.add(self.ducks[0].position);

      const lookat = new THREE.Vector3(
        self.ducks[0].position.x + 2 * Math.sin(self.ducks[0].direction),
        self.ducks[0].position.y,
        self.ducks[0].position.z + 2 * Math.cos(self.ducks[0].direction),
      );
      self.camera.lookAt(lookat);
    } else if (self.pov === POV.TOP_DOWN) {
      self.camera.position.set(
        -Math.sin(self.ducks[0].direction) * 5,
        30,
        -Math.cos(self.ducks[0].direction) * 5,
      );
      self.camera.position.add(self.ducks[0].position);

      const lookat = new THREE.Vector3(
        self.ducks[0].position.x + 0.1 * Math.sin(self.ducks[0].direction),
        self.ducks[0].position.y,
        self.ducks[0].position.z + 0.1 * Math.cos(self.ducks[0].direction),
      );
      self.camera.lookAt(lookat);
    } else {
      self.camera.position.set(
        Math.sin(self.ducks[0].direction + 0.6) * 2,
        1,
        Math.cos(self.ducks[0].direction + 0.6) * 2,
      );
      // self.camera.position.add(self.ducks[0].position);

      const lookat = new THREE.Vector3(
        self.ducks[0].position.x,
        self.ducks[0].position.y + 1,
        self.ducks[0].position.z,
      );

      self.camera.lookAt(lookat);
    }
  }

  /**
   * Handles collisions between duck and bread
   */
  handleCollisions() {
    const self = this;

    function intersect(a: Duck, b: Bread) {
      return (
        a.position.x - a.size.x <= b.position.x + b.size.x &&
        a.position.x + a.size.x >= b.position.x - b.size.x &&
        a.position.y - a.size.y <= b.position.y + b.size.y &&
        a.position.y + a.size.y >= b.position.y - b.size.y &&
        a.position.z - a.size.z <= b.position.z + b.size.z &&
        a.position.z + a.size.z >= b.position.z - b.size.z
      );
    }

    for (let i = 0; i < self.breadList.length; i++) {
      for (let j = 0; j < self.ducks.length; j++) {
        if (intersect(self.ducks[j], self.breadList[i])) {
          self.ducks[j].score++;
          self.scene.remove(self.breadList[i]);

          self.breadList[i] = self.breadList[self.breadList.length - 1];
          self.breadList.pop();
          i--;
          break;
        }
      }
    }
  }

  /**
   * Handles updates based on pressed keys input
   *
   * @remarks
   * `handleInput` does not actually handle an input event,
   * instead, using `pressedKeys` map as the current input state
   */
  handleInput() {
    const self = this;
    const left = self.pressedKeys.get("ArrowLeft") || self.pressedKeys.get("a");
    const right =
      self.pressedKeys.get("ArrowRight") || self.pressedKeys.get("d");
    if ((!left && !right) || (left && right)) {
      self.ducks[0].deltaDirection = 0;
    } else {
      if (left) {
        self.ducks[0].deltaDirection = 3;
      }
      if (right) {
        self.ducks[0].deltaDirection = -3;
      }
    }

    const r = self.pressedKeys.get("r");
    if (r) {
      self.pressedKeys.set("r", false);
      self.pov = (self.pov + 1) % 4;
      if (self.pov === POV.FIRST_PERSON) {
        self.ducks[0].visible = false;
      } else {
        self.ducks[0].visible = true;
      }
    }
  }
}
