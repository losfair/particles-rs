import { RuntimeEnvironment } from "./runtime";
import { ParticlesConfig } from "./particles_misc";

export class Particles {
    private rtEnv: RuntimeEnvironment;
    private config: ParticlesConfig;
    private stateHandle: number;
    private canvas: HTMLCanvasElement;
    private animationHandle: number;

    constructor(rtEnv: RuntimeEnvironment, config: ParticlesConfig, canvas: HTMLCanvasElement) {
        normalizeParticlesConfigInPlace(config, canvas);

        this.rtEnv = rtEnv;
        this.config = config;
        let configInstance = rtEnv.instance.exports.particles_config_create(
            Math.floor(config.height),
            Math.floor(config.width),
            Math.floor(config.nParticles),
            config.maxEdgeLen,
            config.velocityFactor,
            config.nodeRadius
        );
        let stateHandle = rtEnv.instance.exports.particles_state_create(configInstance);
        this.stateHandle = stateHandle;
        this.canvas = canvas;
        this.animationHandle = null;
    }

    destroy() {
        this.rtEnv.instance.exports.particles_state_destroy(this.stateHandle);
        this.stateHandle = 0;
        this.rtEnv = null;
    }

    update() {
        this.rtEnv.instance.exports.particles_state_update(this.stateHandle);
    }

    setSize(height: number, width: number) {
        this.rtEnv.instance.exports.particles_state_set_size(
            this.stateHandle,
            Math.floor(height),
            Math.floor(width)
        );
    }

    _borrowConfig() : number {
        return this.rtEnv.instance.exports.particles_state_borrow_config(this.stateHandle);
    }

    enableCollision() {
        let rtConfig = this._borrowConfig();
        this.rtEnv.instance.exports.particles_config_enable_collision(rtConfig);
    }

    disableCollision() {
        let rtConfig = this._borrowConfig();
        this.rtEnv.instance.exports.particles_config_disable_collision(rtConfig);
    }

    enableEdges() {
        let rtConfig = this._borrowConfig();
        this.rtEnv.instance.exports.particles_config_enable_edges(rtConfig);
    }

    disableEdges() {
        let rtConfig = this._borrowConfig();
        this.rtEnv.instance.exports.particles_config_disable_edges(rtConfig);
    }

    setMagneticStrength(v: number) {
        if(typeof(v) != "number") {
            throw new TypeError("value must be a number");
        }

        let rtConfig = this._borrowConfig();
        this.rtEnv.instance.exports.particles_config_set_magnetic_strength(rtConfig, v);
    }

    setElectricStrength(x: number, y: number) {
        if(typeof(x) != "number" || typeof(y) != "number") {
            throw new TypeError("x & y must be numbers");
        }

        let rtConfig = this._borrowConfig();
        this.rtEnv.instance.exports.particles_config_set_electric_strength(rtConfig, x, y);
    }

    begin() {
        if(this.animationHandle !== null) {
            window.cancelAnimationFrame(this.animationHandle);
            this.animationHandle = null;
        }

        let renderFrame = () => {
            this.animationHandle = null;

            this.update();
            this.render();

            this.animationHandle = window.requestAnimationFrame(renderFrame);
        };

        this.animationHandle = window.requestAnimationFrame(renderFrame);
    }

    stop() {
        if(this.animationHandle !== null) {
            window.cancelAnimationFrame(this.animationHandle);
            this.animationHandle = null;
            return true;
        } else {
            return false;
        }
    }

    render() {
        let output = this.rtEnv.instance.exports.particles_state_render(this.stateHandle);

        let nNodes = this.rtEnv.instance.exports.particles_rendered_get_n_nodes(output);
        let nEdges = this.rtEnv.instance.exports.particles_rendered_get_n_edges(output);
        let nodesHandle = this.rtEnv.instance.exports.particles_rendered_get_nodes_ref(output);
        let edgesHandle = this.rtEnv.instance.exports.particles_rendered_get_edges_ref(output);

        let nodesView = new Float64Array(this.rtEnv.mem.buffer);
        let edgesView = new Float64Array(this.rtEnv.mem.buffer);

        let canvas = this.canvas;
        let canvasCtx = this.canvas.getContext("2d");
        canvasCtx.clearRect(0, 0, canvas.width, canvas.height);

        canvasCtx.fillStyle = this.config.nodeColor;
        canvasCtx.strokeStyle = this.config.lineColor;
        canvasCtx.lineWidth = this.config.lineWidth;
        canvasCtx.globalAlpha = 1.0;

        for(let i = 0; i < nNodes; i++) {
            let base = nodesHandle / 8 + i * 2; // f64Base + i * nFields
            let ptrX = base;
            let ptrY = base + 1;

            let x = nodesView[ptrX];
            let y = nodesView[ptrY];

            canvasCtx.beginPath();
            canvasCtx.arc(y, x, this.config.nodeRadius, 0, 2 * Math.PI);
            canvasCtx.fill();
        }

        for(let i = 0; i < nEdges; i++) {
            let base = edgesHandle / 8 + i * 5; // f64Base + i * nFields
            let ptrLeftX = base;
            let ptrLeftY = base + 1;
            let ptrRightX = base + 2;
            let ptrRightY = base + 3;
            let ptrOpacity = base + 4;

            let leftX = edgesView[ptrLeftX];
            let leftY = edgesView[ptrLeftY];
            let rightX = edgesView[ptrRightX];
            let rightY = edgesView[ptrRightY];
            let opacity = edgesView[ptrOpacity];

            canvasCtx.globalAlpha = opacity;

            canvasCtx.beginPath();
            canvasCtx.moveTo(leftY, leftX);
            canvasCtx.lineTo(rightY, rightX);
            canvasCtx.stroke();
        }

        this.rtEnv.instance.exports.particles_rendered_destroy(output);
    }
}

function normalizeParticlesConfigInPlace(config: ParticlesConfig, canvas: HTMLCanvasElement) {
    if(
        typeof(config.height) != "number"
        || config.height <= 0
    ) {
        config.height = canvas.height;
    }

    if(
        typeof(config.width) != "number"
        || config.width <= 0
    ) {
        config.width = canvas.width;
    }

    if(typeof(config.nParticles) != "number" || config.nParticles < 1) {
        config.nParticles = 50;
    }

    if(typeof(config.maxEdgeLen) != "number" || config.maxEdgeLen <= 0) {
        config.maxEdgeLen = 200;
    }

    if(typeof(config.velocityFactor) != "number" || config.velocityFactor <= 0) {
        config.velocityFactor = 1;
    }

    if(typeof(config.nodeColor) != "string" || !config.nodeColor) {
        config.nodeColor = "#E2F0FF";
    }

    if(typeof(config.lineColor) != "string" || !config.lineColor) {
        config.lineColor = "#E2F0FF";
    }

    if(typeof(config.nodeRadius) != "number" || config.nodeRadius <= 0) {
        config.nodeRadius = 5;
    }

    if(typeof(config.lineWidth) != "number" || config.lineWidth < 0) {
        config.lineWidth = 1;
    }

    config.height = Math.floor(config.height);
    config.width = Math.floor(config.width);
    config.nParticles = Math.floor(config.nParticles);
}
