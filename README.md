# WBP CLI Tool

This repo contains source code for the WBP CLI Tool. This tool will eventually serve many purposes.
As for now the list of purposes can be found below in the app list. Data storage for all of the apps
relys on local text files leaving all of your data in your control making it super easy to backup!

### PlanIt

This app is a bullet journal style planner in the CLI that relys on easy to backup text files!
This allows you to be in control of all of your own data at all times while allowing us to 
save on server costs and prevent us from needing to build out a UI from scratch.

Notes are saved using the following structure.
All Notes are saved in the /Documents directory

Week Notes
Filename - `month.mondayDate.year.weekNotes.txt`
Each line is a note in the format noteID-date-dayOfTheWeek-noteType-isComplete-note-lastUpdated


### MilesOnMiles

Running mileage and and workout tracker that relys on easy to backup text files!
This allows you to be in control of all of your own data at all times while allowing us to 
save on server costs and prevent us from needing to build out a UI from scratch.
