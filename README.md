# Anathema Space War

## Features

- [x] Player can create a game
  - [x] Create splash screen
  - [x] Get the players name
  - [x] Navigate to the start screen
  - [x] Button to create a game
  - [x] API call to create game
  - [x] Navigate the player to the lobby
- [x] Player can join a game
  - [x] Player can enter a game code on the start screen
  - [x] Player can press join game button on start screen if code is entered
  - [x] Send API request to join a game
  - [x] Store API response to game state
  - [x] Player is taken to the lobby
- [x] Players see the game lobby
  - [x] API to get stream of players in lobby
  - [x] All player names are visible in the lobby
  - [x] Each players name is in their chosen color
  - [x] Each players ship is displayed
  - [x] Each players ready status is displayed
- [x] Players can change their color
- [x] Players can change their ship
- [x] Players can ready up
- [x] Players can quit out of the lobby
- [x] When the game switches to playing
  - [x] Stop request for the lobby
  - [x] Start getting the request for the game
  - [x] Switch to game view
- [x] In the lobby, display max ship speed for each ship
- [x] Turn number is displayed in the game
- [x] Players can see their current speed
- [x] Players can set their speed as part of a new turn
- [x] Players can move based on their distance
  - [x] Move button that when pressed highlights legal destinations
  - [x] Clicking on legal destination, set command move_to position and stops highlight
  - [x] Submitting command also sends move to position
  - [x] Upon new turn, ship is set to new turn
- [ ] Players can fire a torpedo
  - [x] Players can see how many torpedoes a ship has when selecting a ship in the lobby
  - [x] Players can see how many torpedoes they have left for their own ship
  - [x] Button to set firing destination
  - [x] While in target setting mode, the cell in the canvas highlights so that I can see where I'll be firing at
  - [x] After setting a target to fire at, the cell remains lit up and the target coordinates are displayed
  - [x] Can cancel setting the target
  - [ ] Add the torpedo firing coordinates to the command

### Polish

- [ ] Upon new turn, ships move smoothly towards new destinations
- [ ] An error should be logged to file instead of the screen when submitting an empty code
