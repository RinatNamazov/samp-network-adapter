# RiNetworkAdapter

SA:MP Client ASI Plugin, which allows to change the address of network adapter.

Supported client versions: 0.3.7 R1/R2/R3/R3-1/R4/R4-2/R5, 0.3.DL R1.

Default adapter address: `127.0.0.1`

The adapter address can be specified in various ways:
* By launching from the launcher [SampX](https://github.com/RinatNamazov/SampX).
* Using the config in the game folder `RiNetworkAdapter.ini`:
```
[ri_network_adapter]
address=127.0.0.1
```
* Can be specified via the command line when starting the game process: `--adapter_address`
* Via C API: `void SetNetworkAdapterAddress(const char* address)`

## Build instructions

```
cargo build --release
```

## License
The source code is published under GPLv3, the license is available [here](LICENSE).
