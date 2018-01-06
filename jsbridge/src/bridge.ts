import { RuntimeEnvironment } from "./runtime";
import { Particles } from "./particles";
import Common from "./common";

(window as any).particles = Object.assign((window as any).particles || {}, {
    RuntimeEnvironment: RuntimeEnvironment,
    Particles: Particles
});

if((window as any).particles._code) {
    Common.code = (window as any).particles._code;
}
