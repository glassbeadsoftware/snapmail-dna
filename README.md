# snapmail-dna

Holochain DNA for [SnapMail](https://github.com/glassbeadsoftware/snapmail-release) from [Glass Bead Software](http://www.glassbead.com/).
To download and use it, go to [snapmail-release](https://github.com/glassbeadsoftware/snapmail-release) repo.

Native application for Windows, Linux and MacOS are available.

Some design documentation is available in the `/spec` folder.

CI and NIX configs are not set up for the moment.


## Building

Get the latest holochain release and nix-shell:
```
nix-shell https://holochain.love
```

To rebuild the DNA for holochain, use the `hc` command:

```
nix-shell --run 'hc package'
```

You should get a DNA hash as the final output:
```
DNA hash: QmaTQtGajbgbnwLhj5LdMs3SwC3XXuktviL25YFbtZmKJF
```

## Testing

To run the tests, Make sure the sim2h url is appropriate in `test\config.js`
Tests can also be enabled/disabled by commenting out the test suites in `test\index.js`

To launch the tests, run command:

```
nix-shell --run 'hc test'
```

## Running with UI

 1. Download the [snapmail-ui repo](https://github.com/glassbeadsoftware/snapmail-ui) and store it at same folder level than `snapmail-dna`
 2. Make sure the sim2h server is up and running:

```
nix-shell --run 'hc sim2h-client -u ws://sim2h.harris-braun.com:9051 -m status'
```
You should get an `Await successfull` response. Otherwise it will timeout.


 3. To run a temporary test agent, do:
```
nix-shell --run 'hc run --networked sim2h --sim2h-server ws://sim2h.harris-braun.com:9051 --agent-name Alice --port 8888'
```

 4. Open your favorite web browser at url: `http://127.0.0.1:8888`

  
  
  
To run a permanent test agent, modifiy the example `conductor-config*.toml` files provided. Than do:

```
nix-shell --run 'holochain -c conductor-config-xxx.toml'
```
