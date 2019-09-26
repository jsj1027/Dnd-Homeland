Start at x level
- Level determines XP
Choose Class
- Consider subclasses
Choose race
Calculate base stats - generate ability scores + race bonus. roll 4 d6, take the best 3 and do that 6 times?
Base stats - strength, dex, constitution, intelligence, wisdom, charisma.
Base stat defaults - [15,14,13,12,10,8]
Auto assign based on race/class. Give more weight to certain stats because of race and class.
Generate Random name or choose name.
Generate random alignment or choose alignment.
Genereate Background. give weight to something that benefits class.
Starting equipment is randomized/chosen based on weight of class/race.
Calculate HP.
Calculate AC
Calculate Size, speed, and languages.


Current list of planned features.
[X] Base stats defaults.
Base stats based on roles - assigned randomly.
[X] Choose class
Choose race
Start at x level.
Allow things to be randomized or chosen. Yaml file?
Give weight to base stats based on class/race.

Feature triage.

Base stat defaults:
[x] I need the program to understand what a character is.
[x] I need the character to have a section to house their stats.
[X] I need their stats to be grabable, and modifyable.
[X] I Need a way to tell the program I want only the base state defaults rather than chance.
[X] I need the program to know what the base default stats are.
[X] I need the program to then randomly assign the base stats values to the character.
[X] I need the program to calculate stat modifiers instantly.

Chose class:
[X] Character Object must understand what classes are.
Possible classes must be listed.
[X] Get random class.
Config file - to choose something and randomize others?
    -use yaml file for easy of use.
    -consider what is customizeable.
Create weight system - stretch goal.
Sub classes - stretch goal.

Character:
Investigate all proper fields for a Character object.
Create a default character with empty fields.



Goal: Full DND Character for noobs
Step 1: Race, Class, Background
Profile:
    Race Name
    Class Name
    Character Name
    Player Name
    Sex
Race
   Name
   Age
   Alignment
   Size
   Speed
   Languages
   Proficienes
    weapons
    tools
Class
    Level
    Experience Points
    HP
        Hit Dice
    Proficiencies
        Armor
        Weapons
        Tools?
        Saving Throws
        Skills
    Equipment
        Equipment
        weapons
        armor
        personal items
        money
Stats
    Ability Scores
        Ability Modifer
    Proficiency Modifer
    Saving Throws Bonus
    Skill scores
    PAssive Perception?
    Armor Class
Attacks and Spells
    Physical Weapons
        Attack mod
Features
    Class features
    race features
    background features
Traits??
    Ideals?
    Personality traits?
    Bonds?
    Flaws?
