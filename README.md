# Save Game Extractor | [Download](https://github.com/popcar2/SaveGameExtractor/releases/)
Save Game Extractor is a tool that automatically locates and copies save files for Windows games in public directories. Basically, it's a tiny program for quickly backing up your save files, without having to look for them one by one!

# What Save Game Extractor doesn't do
This program finds only saves in public directories, such as Documents/My Games/. Luckily, **most** games follow this trend.

There are 3 types of saves that can't be located with this program:
* Games that store their saves in launchers (such as Steam and Ubisoft Connect). Usually these are stored on the cloud, so it should be fine to ignore them.
* Games that store their saves in the game's installation folder. These vary wildly, so it's impossible to track them.
* Games that store their saves in the Windows Registry. I have no idea why some people do this.

# How to use
## Linux

Extract the tar file, then run ./save_locator

**It is highly recommended to run it as sudo to avoid access denied errors**

## Windows

Extract the rar file then run save_locator.exe

# How you can help

Games and their respective save files are ordered in the [save locations text file](https://github.com/popcar2/SaveGameExtractor/blob/master/save_locations.txt). I compiled this by hand from a couple of different sources. Needless to say, it's not 100% complete. **If you know a game that isn't on the list and is in a public directory, please suggest adding it [by making an issue!](https://github.com/popcar2/SaveGameExtractor/issues)**

Big shoutout to the contributors at [PCGamingWiki](https://www.pcgamingwiki.com/wiki/Home), which is where I got most of these from.
