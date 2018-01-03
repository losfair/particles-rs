import { RuntimeEnvironment } from "./runtime";
import { Particles } from "./particles";

(window as any).particles = {
    RuntimeEnvironment: RuntimeEnvironment,
    Particles: Particles
};
