## Build

Run `./build.sh` to compile the actor, after compiled you should see the "sample_actor.wasm" file in "target/wasm32-unknown-unknown/release/" directory.

## Test with runtime

After compiled the actor, you can copy the "sample_actor.wasm" file into [dev-runner](https://github.com/tearust/dev-runner) and follow the readme to test inside the runtime.

If actor deployed successfully, you can use the following command to test http interface defined in "impl" subdirectory:
```bash
curl -H "Content-Type: application/json" -d '{"actor": "someone.sample", "address": "0x0000000000000000000000000000000000000000"}' http://localhost:8000/say-hello
```
and then you will see the output like:
```bash
"Hello world!"
```