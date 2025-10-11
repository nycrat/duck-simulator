# duck simulator

![duck simulator](/menu.png)

## messages

client sends:

- "join_game" (name, variety, color)
- "vote_start_game" ()
- binary_update (DuckProto)

game actor sends to client websocket:

- "re:join_game" (id)
- "cast:start_game" ()
- "cast:end_game" ()
- "cast:new_duck_joined" (id, name, variety, color)
- "cast:bread_spawn" (position3)
- cast:binary_update_world (UpdateSyncProto)

player actor sends to game server actor:

- JoinGame (name, variety, color)
- VoteStartGame ()
- Update (position2)

game server actor sends to player actor:

- re:JoinGame (id)
- StartGame ()
- BreadSpawn (position3)
- UpdateWorld (UpdateSyncProto)
- EndGame ()
