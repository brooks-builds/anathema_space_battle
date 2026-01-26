# Anathema Space War

## Screens

### Splash

- [ ] Set our name
- [ ] Join a game via code
- [ ] Host a game

### Lobby

- [ ] See other players waiting in the lobby
- [ ] Choose what ship we'll be playing as
- [ ] See what ships the other players chose
- [ ] Set our ship color
- [ ] Mark ourselves as ready
- [ ] Leave lobby to return to splash screen

### Play

- [ ] See how long before our turn is skipped
- [ ] Resign early
- [ ] Take move action
- [ ] Take an action
  - [ ] Fire
  - [ ] Scan
- [ ] See what our stats are
- [ ] See who is still taking their turn
- [ ] Watch the turn play out

### Game Over

- [ ] See list of players
- [ ] See list of stats for each player
  - [ ] shots fired
  - [ ] accuracy
  - [ ] distance
  - [ ] damage taken
- [ ] nav back to splash screen
- [ ] quit game

## Networking

Using the Jack Box multiplayer pattern. Instead of user accounts, host can send their name along with a request to start a game. They get a code back that can be shared with friends to join the game.

### Splash

- Join game / create game
  - POST with name, and/or code -> lobby SSE
    - One time
      - player
        - name
        - ship choice
        - color choice
    - ongoing
      - player name joined
        - name
        - ship choice
        - color choice
      - player ship changed
      - player color changed
      - player readied
      - player left game
      - player no longer ready
      - game starting in x seconds
- play game
  - Ongoing SSE
    - When play starts
      - confirmation that game is created
    - While choosing what to do
      - board status
      - player status
      - Next turn at??
      - Other players ready status
      - players dropped
    - While waiting for others to choose
      - player status
      - player dropped
    - Turn simulated
      - All players actions
      - All results from actions
      - Game status (still playing or game over)
  - POST turn data
    - OK, turn locked in
  - POST Quit
