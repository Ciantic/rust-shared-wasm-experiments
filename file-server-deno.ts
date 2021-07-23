import { Application, send } from "https://deno.land/x/oak/mod.ts";

const app = new Application();

app.use(async (context) => {
    // Required for SharedArrayBuffer, see
    // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer
    context.response.headers.append("Cross-Origin-Embedder-Policy", "require-corp");
    context.response.headers.append("Cross-Origin-Opener-Policy", "same-origin");

    await send(context, context.request.url.pathname, {
        root: `${Deno.cwd()}`,
        index: "index.html",
    });
});

console.log("Start listening on http://localhost:8000");
if (Deno.build.os === "windows") {
    Deno.run({
        cmd: ["explorer", "http://localhost:8000"],
    });
}
await app.listen({ port: 8000 });
 