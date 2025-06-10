# erc_substream_2_project Substreams modules

This package was initialized via `substreams init`, using the `evm-minimal` template.

## Usage

```bash
substreams build
substreams auth
substreams gui       			  # Get streaming!
```

Optionally, you can publish your Substreams to the [Substreams Registry](https://substreams.dev).

```bash
substreams registry login         # Login to substreams.dev
substreams registry publish       # Publish your Substreams to substreams.dev
```

## Modules

### `map_my_data`

This module extracts small bits of block data, and does simple computations over the 
number of **transactions** in each block.
