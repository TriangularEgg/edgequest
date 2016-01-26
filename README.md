# edgequest
Roguelike in Python based on the [Roguebasin Tutorial](http://www.roguebasin.com/index.php?title=Complete_Roguelike_Tutorial,_using_python%2Blibtcod) but with several key differences, namely being the feature to store items and monsters inside of JSON files, making modular additions possible without needing to alter the python source in any way.

Some say making a roguelike in python is pointless, due to the slow speeds that may come as a part of constant A* pathfinding, FOV, djikstra maps, and dynamic lighting, but to the naysayers I say "Eh, you're probably right"

## Requirements
1. Python 2.7
2. Enough edge to cut yourself on
3. 10 minutes of free time

## Run Edgequest

#### Linux

It should be as simple as running:

`python2.7 edgequest.py`

in the edgequest directory from a terminal

All imports are from the standard library, so you should have no problem with running this on a linux machine

If it doesn't work try running

`sudo apt-get install python-simplejson`

`sudo apt-get --reinstall libsdl1.2debian`

#### Windows

Currently does not run on windows, but you will need the 32 bit version of python2.7 to start.

## Wiki

Please check the [wiki](https://github.com/TriangularEgg/edgequest/wiki) for further information
