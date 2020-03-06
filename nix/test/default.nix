{ pkgs }:
let
  script = pkgs.writeShellScriptBin "snapmail-test"
  ''
  set -euxo pipefail
  mkdir dist
  hc package -o dist/snapmail.dna.json
  hc test --skip-package
  '';
in
{
 buildInputs = [ script ];
}
