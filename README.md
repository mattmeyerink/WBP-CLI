# WBP CLI Tool

This repo contains source code for the WBP CLI Tool. This tool will eventually serve many purposes.
As for now the list of purposes can be found below in the app list. Data storage for all of the apps
relys on local text files leaving all of your data in your control making it super easy to backup
in whatever way works best for you!

### PlanIt

This app is a bullet journal style planner that contains the following note templates.

- Year View: This is a list based view with a list for each month in a given year. Intended for
high level tasks, events, and notes.
- Month List View: This is a list view of a specific month meant to be updated at the begining of each month.
This contains tasks and notes that are not yet bound by a specific date but that you know will be 
relevant in the coming month.
- Month Highlight View: This is an overview of key events that occured/will occur in a given month. Only 
note allowed per day so choose your words carfully!
- Week View: This is a list based view with a list for each day. This is likely where you will spend most of your time as it contains granular tasks, notes, and events relevant to a specific day.

#### Note Types

`o` : Indicates a task that has not yet been completed.
`x` : Indicates a task that has been completed.
`-` : Indicates a note or thought that has no particular action attached to it.
`*` : Indicates an event.

#### Data Structure

Notes are saved using the following structure.
All Notes are saved in the `/Documents/wbp-data/plan-it` directory

Week Notes
Filename - `{{month}}-{{mondayDate}}-{{year}}-WeekNotes.txt`

Month List View Notes
Filename - `{{month}}-{{year}}-MonthNotes.txt`

Month Highlights 
Filename - `{{month}}-{{year}}-MonthHighlights.txt`

Year Notes
Filename - `{{year}}-YearNotes.txt`

### RecipeCentral

Potentially in the works. TBD.

### MilesOnMiles

Potentially in the works. TBD.
