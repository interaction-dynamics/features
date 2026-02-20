After modifying this folder, you should run the following command to update the snapshots:

```
pnpm install

pnpm test 

cd ../../tools/cli

cargo run  ../../examples/tests-skip-changes/src --project-dir ../../examples/tests-skip-changes --skip-changes --json > tests/snapshots/tests_skip_changes.json

```
