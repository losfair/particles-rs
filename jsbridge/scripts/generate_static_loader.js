const fs = require("fs");
const path = require("path");

const binaryPath = process.argv[2];
const base64Lib = fs.readFileSync(
    path.join(__dirname, "res/base64js.min.js"),
    "utf-8"
);

let data = fs.readFileSync(binaryPath).toString("base64");
console.log(`
(function() {

let module = {
    exports: {}
};
let exports = module.exports;

${base64Lib}

window.particles = Object.assign(window.particles || {}, {
    _code: module.exports.toByteArray("${data}")
});

})();

`);
