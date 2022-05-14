- The frontend is built with [Yew](https://yew.rs/) using [Trunk](https://trunkrs.dev/) and
  embedded into the binary.

For quick frontend work use:`trunk watch` to autorebuild, the embedded files will auto update
Use regular cargo commands for everything else with an additional `trunk build` for final releases.
