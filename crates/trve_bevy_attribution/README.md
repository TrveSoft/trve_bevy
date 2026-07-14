# Trve Bevy Attribution

> Utilities to load standardized attribution and licensing metadata files for game assets.

## Usage

Add `AttributionPlugin` to your app before any other plugins that will load assets in your game. Whenever you load folders, the plugin will check for `AssetEvent<AttributionAsset>` and add their attribution metadata to an `Attributions` resource. Loading these files individually should work too.

The key used in the `Attributions` resource will be the asset's path. If the asset has no path for some reason, the `name` key will be used for this purpose as a default.

## Attribution Metadata Files

These files should be put alongside your assets, with an `.attr.toml` extension. Here's an example:

```toml
[meta]
# Asset name (Required).
name = "Asset Name"
# Author's name (Required).
author = "Author Name"
# License/s applicable to this asset. Use a list even if a single license applies (Required).
license = ["CC-BY-SA 3.0", "GPL 2.0"]
# Link to the source (Optional).
link = "https://asset.com/"
# Extended description. Useful for crediting additional people, custom attributions or stating modifications (Optional).
description = "Some optional description."
```

Missing or empty (i.e. present keys with empty strings) optional fields will have a value of `None` when parsed.

While in development mode, the plugin will issue warnings if a wrong license type is used. Valid license types are listed in the `VALID_LICENSES` constant.

## Bevy compatibility

| `bevy` Version | `trve_bevy_attribution` Version |
| -------------- | ------------------------------- |
| 0.19           | 0.1, 0.2                        |
