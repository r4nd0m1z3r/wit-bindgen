echo "Compile: start"
nim c -o:test.wasm test.nim
compile_exit_code=$?
if [ $compile_exit_code -ne 0 ]; then
    echo "Compile failed, exit code: $compile_exit_code"
    exit $compile_exit_code
fi
echo "Compile: done"

echo "Embed: start"
wasm-tools component embed ../../../tests/codegen/wasi-cli/wit/ -w wasi:cli/command test.wasm -o test.component.wasm
embed_exit_code=$?
if [ $embed_exit_code -ne 0 ]; then
    echo "Embed failed, exit code: $embed_exit_code"
    exit $embed_exit_code
fi
echo "Embed: done"

wasm-tools component new test.component.wasm -o test.component.wasm
new_component_exit_code=$?
if [ $new_component_exit_code -ne 0 ]; then
    echo "New component failed, exit code: $new_component_exit_code"
    exit $new_component_exit_code
fi

echo "Run: start"
wasmtime run test.component.wasm
run_exit_code=$?
if [ $run_exit_code -ne 0 ]; then
    echo "Run: failed, error code: $run_exit_code"
else
    echo "Run: done"
fi
