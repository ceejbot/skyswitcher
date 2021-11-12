# skyswitcher

Switch to the version of the Skyrim & SKSE executables you prefer on the fly, and launch the game. Assumes you're using SKSE!

Inspired by [Skyrim Lite Loader](https://www.nexusmods.com/skyrimspecialedition/mods/58271) and uses the same directory layout.

Put the executable into your Steam `Skyrim Special Edition` folder. Then set up the following directory structure:

```sh
➜ tree SLL
Skyrim Special Edition

SLL
├── Anniversary
│  ├── skse64_loader.exe
│  └── SkyrimSE.exe
└── Legacy
   ├── skse64_loader.exe
   └── SkyrimSE.exe
```

Put the appropriate executable versions into each folder. The version of SKSE should match the Skyrim edition. `skse64_1_5_97.dll`, `skse64_1_6_318.dll`, and `skse64_steam_loader.dll` should be in your Skyrim directory. This is identical to the one SLL wants!

Then run either `skyswitcher legacy` or `skyswitcher anniversary` to execute the one you want. You can make these MO2 execs to run in an MO2 context.

```sh
skyswitcher 0.1.0
swap versions of skyrim & skse on the fly

USAGE:
    skyswitcher [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Pass -v or -vv to increase verbosity

SUBCOMMANDS:
    anniversary    Run the Anniversary Edition
    help           Prints this message or the help of the given subcommand(s)
    legacy         Run the legacy Special Edition
```

## License

[Blue Oak Model License](https://blueoakcouncil.org/license/1.0.0); text in [LICENSE.md](./LICENSE.md).
