# Rustaxion

## About
🚧🚧🚧🚧 <br />
A WIP reverse engineered implementation of [音灵 INVAXION](https://store.steampowered.com/app/921630/_INVAXION/)'s servers, based on the client code.

If you are looking for a working solution, try out my [server emulator](https://github.com/Invaxion-Server-Emulator/invaxion-server-emulator) implemented as a client-side mod for the game.

## Technologies
Here I list some of the crates I found useful
- tokio's [runtime](https://github.com/tokio-rs/tokio) as well as its helpers to run a TCP nice server.
- tokio's [prost](https://github.com/tokio-rs/prost) is used to encode/decode protobuf messages.
- [enum-repr-derive](https://github.com/ssalonen/enum-repr-derive) to encode/decode enums outside of protobuf.
- [anyhow](https://github.com/dtolnay/anyhow) to make error handling more convenient


## Contributing
Feel free to contribute in any way you can! <br />
I even accept [donations](https://github.com/sponsors/ArjixWasTaken) if you want to contribute but cannot code :^)


## Attributions
1. [invaxion-server](https://github.com/603185423/invaxion-server) by @MoeGrid and @603185423 for the server logic
2. [Invaxion-Server-Emulator](https://github.com/Invaxion-Server-Emulator/invaxion-server-emulator) by @ArjixWasTaken (me!) for discovering the missing parts (CosmicTour and some other stuff)
