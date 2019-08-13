# Rusty Clash Royale SDK

This is my first rust project.

It's currently feature-incomplete on purpose because I assume the way that I've done things is terrible, so when I figure out how to do things better it will be a lot less to refactor.

The idea is to provide a client that will return appropriately typed structs for every API interaction.

## Usage

If you for some reason want to use the two methods available in this client, you can do so!

Note that clan_id has to be prepended with `%23` instead of `#`. I'll update the code to handle either eventually.

```rust
use clash_royale::ClashClient;
let mut clash = clash_royale::ClashClient { token: token.to_owned() };
let res = clash.get_clan_warlog( "clan_id");
```
