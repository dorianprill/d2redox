# Diablo II Reverse(d) Engine
Toy project botting engine for learning rust & reverse engineering with one of my favorite games ever.  

As of now, this project just functions as a simple Diablo2 game server packet sniffer.  
Most core game functionality will probably be split off into an own library `d2corelib`, ideally also usable for webassembly targets. 

## Design Principles:
  - cross platform without runtime/VM requirement  
  - speed, to be (i.e. pick) faster than other bots
  - low overhead to potentially run many instances
  - do not modify game memory, instead read and write raw network packages  
  - reproduce game state as accurately as possible/needed
  - enable clientless game connections 


## Milestones:
1. read raw network packages, filter by port, identify protocol for each package and pass them on to their specific handlers  
   - [x] D2GS (plain)  
   - [x] D2GS (compressed)  
   - [ ] BNCS  
   - [ ] MCP  
   - [ ] Serialize decoded D2GS Packets and send to pimap for rendering
2. Track the needed game state in own data structures.  
   - [ ] Items (Ground, Inventory, Stash, Cube, Belt)
   - [ ] Players and party  
   - [ ] Allow packets to be injected on event e.g. gold drops in pick range -> auto pick it (need to track above game state for this)
   - [ ] Quests  
3. Priority based event handling for modules (e.g. chicken, pickit, \[move, combat, ...\])
   - [ ] Chicken module
   - [ ] Pickit module (parse configuration from .nip files, see kolbot)
4. Generate game maps from game seed
   - [ ] Implement pathing (to walk/teleport to location on the same map) 
   - [ ] Navigate to arbitrary locations in the game, considering found waypoints, quest state
5. allow scripting with gluon, dyon or lua/wren?

## How to Build
### Linux
`cargo build --release`
### Mac Os
`cargo build --release` (not tested yet)
### Windows
You will need to install npcap and additionally download the WinPcap Developers Pack as per the [libpnet](https://github.com/libpnet/libpnet) build instructions for Windows. Then point your user environment variable `LIB` (create if nonexistent) to the folder where to find Packet.lib i.e. `WpdPack/Lib/x64/` from the WinPcap Developers Pack you just downloaded. Then `cargo build --release`
This should get the project building but the executable crashes while querying the available network interfaces on my machine (maybe need to adapt code for windows).    

## How to Run
1. Launch a Diablo II client.
2. Execute `target/release/d2re` which will launch a command window
3. Connect to b.net and watch the output of the packet sniffer in the command window (packet id + payload as hex)

### Disclaimer
Little of this works yet and most probably never will. Unless you feel like **contributing, which is welcome**, it is mostly an exercise. 
Heavily inspired by an old [client-less C# bot by dkuwahara](https://github.com/dkuwahara/OmegaBot) and a [blog post by Eric Carmichael](http://www.ericcarmichael.com/my-diablo-2-botting-phase.html) and, of course, [D2BS](https://github.com/noah-/d2bs). Another good resource is the [diablo 2 protocol js library](https://github.com/MephisTools/diablo2-protocol).
