{ pkgs }:
let
  script = pkgs.writeShellScriptBin "snapmail"
  ''
  set -euxo pipefail
  holochain -c ./conductor-config.toml
  '';
in
{
 buildInputs = [ script ];
}
