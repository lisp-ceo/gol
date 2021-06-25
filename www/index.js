import { Universe } from "gol";

const pre = document.getElementById("canvas");
const u = Universe.new();

const renderLoop = () => {
    pre.textContent = u.render();
    u.tick();

    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);
