# <img src="docs/war.png" width="40%" align="right"> war

[![pipeline status](https://gitlab.com/whatisaphone/war/badges/master/pipeline.svg)](https://gitlab.com/whatisaphone/war/commits/master)

A tool that lets you view/edit [Darksiders] save files and game assets.

[Darksiders]: https://en.wikipedia.org/wiki/Darksiders_(video_game)

>Darksiders Warfinitive Edition - All your save game are belong to us!

You can download the latest version from the [releases] page.

[releases]: https://github.com/whatisaphone/war/releases

## How to use

You have a couple options:

1. [View the worlds/scripts online][browse]. Nothing to download, nothing to install.

   [browse]: https://github.com/whatisaphone/war/tree/browse/browse

2. Run `war --help` for the full list of options.

3. Continue reading below for the guided tour.

    - [Save editing](#save-editing)
    - [Asset extracting](#asset-extracting)

### Save editing

Which edition of the game do you have?

<details>
<summary><b>OG</b></summary>

Your saves should be in a path that looks something like this:

    C:\Users\<username>\Documents\My Games\Darksiders\<steam-user-id>\dsauto.dsav

- Run `war dsav decode <path-to-dsav>` to decode a save from `.dsav` to `.json`.
- The newly-created `.json` will have an unpacked version of your save. You can edit this file.
- Run `war dsav encode <path-to-json>` to encode back to `.dsav`.
- To use the new save, just quit to the main menu and choose "Load Game".

</details>

<details>
<summary><b>Warmastered</b></summary>

Your saves should be in a path that looks something like this:

    C:\Program Files (x86)\Steam\userdata\<steam-user-id>\462780\remote\autocloud\save\keen_savegame_0_0.sav

The file contains all 10 save slots (including the autosave slot), but this tool only reads/writes the autosave slot.

- Run `war sav decode <path-to-sav>` to decode the autosave to `.json`.
- The newly-created `.json` will have an unpacked version of your save. You can edit this file.
- Run `war sav encode <path-to-sav> <path-to-json>` to write back to the `.sav`. The autosave slot will be overwritten, and the other save slots will be left alone.
- If the game is running, you'll need to exit and restart for it to read the new data. Unfortunately, this edition only reads the save file once when the game first loads.

</details>

⚠ **NOTE** (both editions): Darksiders will refuse to load a save if the `SaveGameInfo`'s `UserId` does not match the logged-in Steam user's ID. If you want to use a save on a different account than created it, the `UserId` property must be changed to match.

### Asset extracting

This app can extract the game's various assets to a readable format. It cannot re-pack them into the original format. These command work just as well on files from both OG and Warmastered.

#### Manifest

Most of the game's assets (meshes, textures, audio, etc.) are stored in resource packages (`*.oppc`) which are referenced by the manifest (`media/pc.mnfst`).

To extract these, point the tool at your game directory (either `Darksiders` or `Darksiders Warmastered Edition`).

```sh
war manifest packages unpack <path-to-Darksiders> <output-dir>
```

The unpacked files will take up about 17GB (for the Warmastered Edition).

#### Worlds

The game's worlds are stored in `/path/to/Darksiders/media/worlds.mnfst`. To extract them, run:

```sh
war worlds decode <path-to-worlds.mnfst> <output-dir>
```

See the [browse] readme for more info on what exactly these files are.
 
#### VisScript

The Darksiders devs used a visual editor for some scripts. You can render them to Graphviz files using:

```sh
war worlds visscript draw <path-to-worlds.mnfst> <output-dir>
```

Take a look at the [browse] branch to see the final results.

#### Scripts

The game's scripts are stored in `/path/to/Darksiders/media/scripts.obsp`. To extract them, run:

```sh
war obsp decode <path-to-scripts.obsp> <output-dir>
```

Congratulations, you did it! Now you have several thousand more files on your hard drive. 🎉

## Development

### Install prerequisites

- [Rust]
- [pre-commit]

[Rust]: https://www.rust-lang.org/
[pre-commit]: https://pre-commit.com/

### Install the pre-commit hook

```sh
pre-commit install
```

This installs a Git hook that runs a quick sanity check before every commit.

### Run the app

```sh
cargo run
```

### Run the tests

```sh
cargo test
```
