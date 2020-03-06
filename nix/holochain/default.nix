{ pkgs }:
let
  script = pkgs.writeShellScriptBin "snapmail-dna"
  ''
  set -euxo pipefail
  holochain -c ./conductor-config.toml
  '';
in
{
 buildInputs = [ script ];
}
