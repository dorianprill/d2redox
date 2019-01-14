toy project botting engine for learning rust & reverse engineering with one of my favorite games ever.  

goals:
  - read network packages and track the needed game state in own data structures.  
  - construct and inject network packages directly to the network interface
  - priority based event handling for modules (e.g. chicken, pickit, \[moving, combat, ...\])
  - generate game maps from game seed and display as ascii  
  - pathing based on generated maps  (to walk/teleport to pick location or gateway)  
  - allow scripting with gluon, dyon or lua
 

disclaimer: none of this works yet and none of it probably ever will work. it's just a haxercise :)
heavily inspired by an old [client-less C# bot by dkuwahara](https://github.com/dkuwahara/OmegaBot) and a [blog post by Eric Carmichael](http://www.ericcarmichael.com/my-diablo-2-botting-phase.html) and, of course, [D2BS](https://github.com/noah-/d2bs).
