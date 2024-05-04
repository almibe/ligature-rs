import {existsSync} from "https://deno.land/std/fs/mod.ts";
export {}

if (existsSync("./pkg/package.json")) {
    const templatejson = JSON.parse(await Deno.readTextFile("./template_package.json"));
    const packagejson = JSON.parse(await Deno.readTextFile("./pkg/package.json"));
    
    for (let key in templatejson) {
        packagejson[key] = templatejson[key];
    }
    Deno.writeTextFile("./pkg/package.json", JSON.stringify(packagejson, null, 2));
} else {
    console.log("pkg/package.json does not exist!");
}
