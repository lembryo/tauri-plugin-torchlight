import { readFileSync } from "fs"
import { fileURLToPath } from "url"
import typescript from "@rollup/plugin-typescript"

const pkg = JSON.parse(
    readFileSync(fileURLToPath(new URL("./package.json", import.meta.url)), "utf8"),
)

export default {
    input: "guest-js/index.ts",
    output: [
        {
            file: pkg.exports.import,
            format: "esm",
        },
        {
            file: pkg.exports.require,
            format: "cjs",
        },
    ],
    plugins: [
        typescript({
            declaration: true,
            declarationDir: "./dist-js",
            rootDir: "guest-js",
        }),
    ],
    external: [/^@tauri-apps\/api/, ...Object.keys(pkg.dependencies ?? {})],
}
