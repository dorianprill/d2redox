# d2ng

Toy packet injection engine for learning rust & reverse engineering with one of my favorite games ever.  

As of now, this project just functions as a simple Diablo2 game server packet sniffer.  
> Note: Most game related code has been organized into a standalone library [libd2r](https://github.com/dorianprill/libd2r) to support this and other tools

## Goals

- cross platform without runtime/VM requirement  
- low overhead to potentially run many instances
- do not modify game memory, instead read and write raw network packages  
- reproduce game state as accurately as possible/needed
- enable clientless game connections

## Milestones

1. Read, Decode, and Print network packages for Game-, Realm-, and BNetChat servers (this is now entirely done in `libd2`, see repo)
   - [x]
2. Priority based event handling for passive modules (e.g. chicken, pickit, [move, combat, ...])
   - [ ] Chicken module
   - [ ] Pickit module (parse configuration from .nip files, see kolbot)
3. Pathing and Collision
   - [ ] Implement pathing (to walk/teleport to location on the same map)
   - [ ] Navigate to arbitrary locations in the game, considering found waypoints, quest state
4. allow scripting with gluon, dyon, lua, wren, TypeScript?
   - [ ] TBD

## How to Build

### Linux

Tested with Diablo 2 (Legacy) and WINE
`cargo build --release`

### Mac Os

`cargo build --release` (not tested yet)

### Windows

You will need to install `ncap` or the `WinPcap Developers Pack` as per the [libpnet](https://github.com/libpnet/libpnet) build instructions for Windows (I tested the latter). Then point your user environment variable `LIB` (create if nonexistent) to the folder where to find Packet.lib i.e. `WpdPack/Lib/x64/` from the WinPcap Developers Pack you just downloaded. Then `cargo build --release`
This will get the project building.  
Currently, in order to find the internet-connected network interface, it is necessary to disable disconnected-but-enabled interfaces (such as virtual adaperts for VPN).

## How to Run

1. Launch a Diablo II client.
2. Execute `target/release/d2ng` which will launch a command window
3. Connect to b.net and watch the output of the packet sniffer in the command window (packet id + payload as hex)

## Disclaimer

Little of this works yet and most probably never will. Unless you feel like **contributing, which is welcome**, it is mostly an exercise.
Heavily inspired by an old [client-less C# bot by dkuwahara](https://github.com/dkuwahara/OmegaBot) and a [blog post by Eric Carmichael](http://www.ericcarmichael.com/my-diablo-2-botting-phase.html) and, of course, [D2BS](https://github.com/noah-/d2bs). Another good resource is the [diablo 2 protocol js library](https://github.com/MephisTools/diablo2-protocol).
