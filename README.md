# devis
This is my personal cli tool that I pack all of my common workflows into.

## Idea for this
I have always liked building small little tools here and there. I have always wanted
to do something like this. [This bluesky post](https://bsky.app/profile/alilleybrinker.com/post/3lipylh46ic2i)
 pushed me to actually do it. 

## Currently working on...
- note - implementation done!
    - add config file reading and generation
    - add clear and clean logging and better error handling
    - test the things please...

## Ideas
- create reminders and TODOs - some kind of local storage
    - utilize colors and such for priorities
    - would be cool to show top 3 things in list with every response to a command
- get a list of things on my schedule

## Scratch Thoughts
- **as soon as we need a config file for anything, create a utility to generate config file**
- utilize cfg build stuff for different OSs
- utilize indicatif (or others) crate for progress bars and such
- utilize clap-verbosity-flag crate for quick implementation of verbosity flag

## devcontainer
that is used so i can actually develop on a windows machine utilizing vscode