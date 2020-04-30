# snapmail-dna

[![CircleCI](https://circleci.com/gh/h-be/snapmail-dna.svg?style=svg)](https://circleci.com/gh/h-be/snapmail-dna)

Holochain DNA for SnapMail from [Glass Bead Software](http://www.glassbead.com/), see [snapmail-ui](https://github.com/ddd-mtl/snapmail-ui) for main use app.

## Building

To rebuild the DNA that holochain uses to run use the `hc` command:

```
nix-shell --run 'hc package'
```

Stop the running conductor (ctrl + c) and rerun the above again if you make changes to the DNA.

## Testing

To run the tests

```
nix-shell --run snapmail-test
```

## Running

FIXME
