const fs = require("fs");
const uglifyjs = require("uglify-es");

[
    "bridge.js",
    "particles-code.js"
].map(v => "build/" + v)
.forEach(filePath => {
    console.log("-> " + filePath);
    try {
        let code = fs.readFileSync(filePath, "utf-8");
        let result = uglifyjs.minify(
            code,
            {
                mangle: {
                    toplevel: true
                }
            }
        );
        if(result.error) {
            throw result.error;
        }
        let output = result.code;
        fs.writeFileSync(filePath, output);
    } catch(e) {
        console.log(e);
    }
});
