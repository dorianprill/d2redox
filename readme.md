# Diablo II Reverse(d) Engine
toy project botting engine for learning rust & reverse engineering with one of my favorite games ever.  

## Design Principles:
  - cross platform - no runtime or VM required  
  - speed, to be (i.e. pick) faster than other bots
  - do not modify game memory, instead read and write raw network packages  
  - reproduce game state as accurately as possible
  - conceptually allow for clientless botting   

## Milestones:
1. read raw network packages and filter by port
   - identify protocol for each package (D2GS, BNCS, MCP) and pass them on to their specific handlers  
2. Track the needed game state in own data structures.  
  - Items (Ground, Inventory, Stash, Cube, Belt)
  - Players and party
  - Quests
3. Priority based event handling for modules (e.g. chicken, pickit, \[move, combat, ...\])
  - Write chicken module
  - Write Pickit module (parse configuration from .nip files, see kolbot)
4. Generate game maps from game seed
  - Implement pathing (to walk/teleport to pick location or gateway)
  - Maybe display as ASCII in console 	 
5. allow scripting with gluon, dyon or lua

## How to Run
### Linux
Just build it with cargo and run the executable, should work right out of the box
### Mac Os
Not tested yet
### Windows
You will need to install WinPcap or npcap  and download the WinPcap developer tools as per the [libpnet](https://github.com/libpnet/libpnet) build instructions for Windows. Then point your `$Env:LIB` to the Folder where to find Packet.lib i.e. `WpdPack/Lib/x64/Packet.lib`.
I got it building with npcap+Packet.lib but the executable crashes when trying to find Packet.lib so further steps are necessary.

### Disclaimer
Little of this works yet and most probably never will. Unless you feel like **contributing, which is welcome**, it remains a haxercise :)  
Heavily inspired by an old [client-less C# bot by dkuwahara](https://github.com/dkuwahara/OmegaBot) and a [blog post by Eric Carmichael](http://www.ericcarmichael.com/my-diablo-2-botting-phase.html) and, of course, [D2BS](https://github.com/noah-/d2bs).
