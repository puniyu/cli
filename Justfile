set windows-shell := ["powershell.exe", "-c"]
set shell := ["bash", "-cu"]

build:
    just puniyu_cli/build
node-build:
    just puniyu_cli_node/build