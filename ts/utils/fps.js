const THOUSAND = 1000;
export default class FPS {
    constructor() {
        this.node = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
        this.node.textContent = "FPS: ";
    }
    render() {
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const frameLength = (1 / delta) * THOUSAND;
        this.frames.push(frameLength);
        if (this.frames.length > 100) {
            this.frames.shift();
        }
        const fps = this.frames.reduce((sum, frame) => sum + frame) / this.frames.length;
        this.node.textContent = `FPS: ${Math.round(fps)}`;
    }
}
