trait Areas {
  const areas: & String[];
} 


impl Areas for Unknown {
    const areas = ["Unknown"];
}

struct Act1;
// Act 1, cow level counts as Act 1
impl Areas for Act1 {
  const areas = ["Rogue Encampment", "Blood Moor", "Cold Plains", "Stony Field", "Dark Wood",
                "Black Marsh", "Tamoe Highland", "Den Of Evil", "Cave Level 1",
                "Underground Passage Level 1", "Hole Level 1", "Pit Level 1",
                "Cave Level 2", "Underground Passage Level 2", "Holy Level 2",
                "Pit Level 2", "Burial Grounds", "Crypt", "Mausoleum", "Forgotten Tower",
                "Tower Cellar Level 1", "Tower Cellar Level 2", "Tower Cellar Level 3",
                "Tower Cellar Level 4", "Tower Cellar Level 5", "Monastery Gate",
                "Outer Cloister", "Barracks", "Jail Level 1", "Jail Level 2",
                "Jail Level 3", "Inner Cloister", "Inner Cloister 2", "Catacombs Level 1",
                "Catacombs Level 2", "Catacombs Level 3", "Catacombs Level 4", "Tristram",
                "The Secret Cow Level"];
}

    
struct Act2;    
impl Areas for Act2 { 
    const areas = ["Lut Gholein", "Rocky Waste", "Dry Hills", "Far Oasis", "Lost City",
                  "Valley Of Snakes", "Canyon Of The Magi", "Sewers Level 1",
                  "Sewers Level 2", "Sewers Level 3", "Harem Level 1", "Harem Level 2",
                  "Palace Cellar Level 1", "Palace Cellar Level 2", "Palace Cellar Level 3",
                  "Stony Tomb Level 1", "Halls Of The Dead Level 1",
                  "Halls Of The Dead Level 2", "Claw Viper Temple Level 1",
                  "Stony Tomb Level 2", "Halls Of The Dead Level 3",
                  "Claw Viper Temple Level 2", "Maggot Lair Level 1", "Maggot Lair Level 2",
                  "Maggot Lair Level 3", "Ancient Tunnels", "Tal Rashas Tomb 1",
                  "Tal Rashas Tomb 2", "Tal Rashas Tomb 3", "Tal Rashas Tomb 4",
                  "Tal Rashas Tomb 5", "Tal Rashas Tomb 6", "Tal Rashas Tomb 7",
                  "Tal Rashas Chamber", "Arcane Sanctuary"];
}

struct Act3;
impl Areas for Act3 {
    const areas = ["Kurast Docks", "Spider Forest", "Great Marsh", "Flayer Jungle",
                  "Lower Kurast", "Kurast Bazaar", "Upper Kurast", "Kurast Causeway",
                  "Travincal", "Archnid Lair", "Spider Cavern", "Swampy Pit Level 1",
                  "Swampy Pit Level 2", "Flayer Dungeon Level 1", "Flayer Dungeon Level 2",
                  "Swampy Pit Level 3", "Flayer Dungeon Level 3", "Sewers Level 1",
                  "Sewers Level 2", "Ruined Temple", "Disused Fane", "Forgotten Reliquary",
                  "Forgotten Temple", "Ruined Fane", "Disused Reliquary",
                  "Durance Of Hate Level 1", "Durance Of Hate Level 2",
                  "Durance Of Hate Level 3"];
}

struct Act4;
impl Areas for Act4 {
    const areas = ["The Pandemonium Fortress", "Outer Steppes", "Plains Of Despair",
                  "City Of The Damned", "River Of Flame", "The Chaos Sanctuary"];
}

struct Act5;
// Act 5, Uber-tristram counts as Act 5
impl Areas for Act5 {
    const areas = ["Harrogath", "The Bloody Foothills", "Frigid Highlands", "Arreat Plateau",
                  "Crystalline Passage", "Frozen River", "Glacial Trail", "Drifter Cavern",
                  "Frozen Tundra", "The Ancients Way", "Icy Cellar",
                  "Arreat Summit", "Nihlathaks Temple", "Halls Of Anguish", "Halls Of Pain",
                  "Halls Of Vaught", "Abaddon", "Pit Of Acheron", "Infernal Pit",
                  "Worldstone Keep Level 1", "Worldstone Keep Level 2",
                  "Worldstone Keep Level 3", "Throne Of Destruction", "Worldstone Keep",
                  "Matrons Den", "Forgotten Sands", "Furnace Of Pain", "Uber Tristram"];
}
