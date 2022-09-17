# Remodel Changelog

## Unreleased Changes

## 0.11.0 (2022-09-16)
* **Breaking:** Reordered parameters of `remodel.writeModelFile` and `remodel.writePlaceFile` to accept path first, then content ([#81]).
* Added `Instance:ClearAllChildren()`. ([#84])
* Added `remodel.removeFile(path)` and `remodel.removeDir(path)`. ([#80])
* Fixed behavior of reading files in `.remodel` directory. ([#83])
* Changed to `rbx_cookie` for authentication. ([#87])

[#80]: https://github.com/rojo-rbx/remodel/pull/80
[#81]: https://github.com/rojo-rbx/remodel/pull/81
[#83]: https://github.com/rojo-rbx/remodel/pull/83
[#84]: https://github.com/rojo-rbx/remodel/pull/84
[#87]: https://github.com/rojo-rbx/remodel/pull/87

## 0.10.0 (2022-06-13)
* Switched from `rlua` to `mlua`, which should improve Lua performance slightly. ([#73])
* Updated all dependencies to improve Roblox compatibility.

[#73]: https://github.com/rojo-rbx/remodel/pull/73

## 0.9.1 (2021-10-11)
* Updated to latest rbx-dom libraries.
* Increased Roblox API request timeout from 30 seconds to 3 minutes. ([#63])

[#63]: https://github.com/rojo-rbx/remodel/pull/63

## 0.9.0 (2021-07-19)
* Updated to rbx-dom 2.0 ecosystem.
* Added Instance:GetFullName() ([#49](https://github.com/rojo-rbx/remodel/pull/49))
* Added `Instance:FindFirstChildOfClass()` ([#50](https://github.com/rojo-rbx/remodel/pull/50))
* Added support for CFrame ([#48](https://github.com/rojo-rbx/remodel/pull/48))
* Added support for Vector3, and improved Vector3int16 ([#46](https://github.com/rojo-rbx/remodel/pull/46))
* Added Color3.fromRGB(red, blue, green) ([#44](https://github.com/rojo-rbx/remodel/pull/44))
* Added `json.toStringPretty` ([#55](https://github.com/rojo-rbx/remodel/pull/55))

## 0.8.1 (2021-04-09)
* Updated to latest rbx_xml, which should fix `OptionalCoordinateFrame`-related issues.

## 0.8.0 (2021-04-09)
* Ported to rbx-dom v2, which includes full support for binary and XML model files.
* Changed all upload commands to upload as binary instead of XML.
* Added support for CSRF negotiation to fix asset uploading. ([#25][#25])
* Added support for Vector3int16.
* Added support for BinaryString values.
* Added Instance:GetDescendants().

[#25]: https://github.com/rojo-rbx/remodel/issues/25

## 0.7.1 (2020-07-06)
* Fixed first argument to `remodel run` script being eaten by Remodel. ([#19](https://github.com/rojo-rbx/remodel/issues/19))

## 0.7.0 (2020-04-19)
* **Breaking**: Moved script execution to `remodel run` to make room for new subcommands.
	* If you previously used `remodel foo.lua`, use `remodel run foo.lua` now.
* Added `--verbose` and `-v` flags for setting verbosity.
* Added `json.fromString` and `json.toString` for encoding/decoding JSON
* Added `remodel.isFile` and `remodel.isDir`.
* Added support for reading the auth cookie through the `REMODEL_AUTH` environment variable.
* Added support for Remodel looking for scripts in the `.remodel` folder of a project
	* `remodel run foo` will now run `.remodel/foo.lua` if it exists.
* Added (experimental) support for building Rojo projects through `rojo.buildProject`.
	* This is behind the `unstable_rojo_api` Cargo feature and is not enabled by default.
* Improved logging and error reporting across the board.

## 0.6.1 (2019-12-11)
* Upgraded reflection database and dependencies.
	* Error messages should now be improved, thanks to an rlua upgrade
	* XML models with CRLF line endings should no longer error spuriously, thanks to an rbx_xml upgrade

## 0.6.0 (2019-09-27)
* **Breaking:** `Instance.new` now only works for instances that actually exist.
* Added `Instance:Clone()` for copying instances all over the place, as is Roblox tradition. ([#12](https://github.com/rojo-rbx/remodel/issues/12))
* Added `DataModel:GetService()` for finding services and creating them if they don't exist, like Roblox does. ([#10](https://github.com/rojo-rbx/remodel/issues/10))
* Added `remodel.getRawProperty(instance, name)`, a clunky but powerful API for reading properties with no validation.
* Added `remodel.setRawProperty(instance, name, type, value)` for writing properties with no validation.
* Fixed Remodel dropping unknown properties when reading/writing XML models. This should make Remodel's behavior line up with Rojo.
* Improved error messages in preparation for [#7](https://github.com/rojo-rbx/remodel/issues/7) to be fixed upstream.
* Remodel Windows binaries now statically link the MSVC CRT, which should improve portability.

## 0.5.0 (2019-09-21)
* Added `Instance.new` for creating instances.
* Added `Instance:Destroy()` for destroying instances instead of just parenting them to nil.
	* Unlike Roblox, no properties can be accessed on a destroyed instance or else Remodel will throw an error. Be careful!
* Added APIs for interacting with models and places on Roblox.com:
	* `remodel.readModelAsset`
	* `remodel.readPlaceAsset`
	* `remodel.writeExistingModelAsset`
	* `remodel.writeExistingPlaceAsset`
	* These APIs will pull your `.ROBLOSECURITY` cookie from Roblox Studio if you're on Windows, or you can pass a cookie explicitly using `--auth [cookie]`

## 0.4.0 (2019-09-18)
* Added `remodel.readDir` for enumerating directories.
* Added early support for `rbxm` models in `remodel.readModelFile` and `remodel.writeModelFile`.
	* When an `rbxm` model is written or read, a warning will be printed to the console.

## 0.3.0 (2019-09-15)
* Added `remodel.writeFile` and `remodel.readFile` for handling regular files.
* Added support for `==` on instances.
* Added support for reading and writing `Parent` on instances.
* Added script file name in error stack traces.

## 0.2.0 (2019-09-14)
* Improved CLI documentation. Try `remodel --help`!
* Added support for extra arguments. They're passed into the script as `...`.
* Added support for reading from stdin. Use `-` as the input file!
	* `echo "print('Hi')" | remodel -`
* **Breaking:** split `remodel.load` into `remodel.readPlaceFile` and `remodel.readModelFile`.
	* `readPlaceFile` can only read `rbxlx` files, and returns a `DataModel` instance.
	* `readModelFile` can only read `rbxmx` files, and returns a list of instances.
* **Breaking:**: split `remodel.save` into `remodel.writePlaceFile` and `remodel.writeModelFile`.
	* `writePlaceFile` can only write `rbxlx` files.
	* `writeModelFile` can only write `rbxmx` files.
	* This split helps Remodel avoid funny tricks to detect what encoding scheme to use.

## 0.1.0 (2019-09-12)
Initial release!

* Basic API for loading and saving places, as well as creating directories
* Single-command CLI that runs a Lua 5.3 script with Remodel APIs
