# duck simulator

![duck simulator](/docs/assets/menu.png)

https://github.com/user-attachments/assets/19cc0436-e54a-4955-bdbb-998be684f3b6

## messages

client sends:

- "join_game" (name, variety, color)
- "vote_start_game" ()
- binary_update (DuckProto)

game actor sends to client websocket:

- "re:join_game" (id)
- "cast:start_game" (start_time, game_duration)
- "cast:end_game" ()
- "cast:join_game" (id, name, variety, color)
- "cast:leave_game" (id)
- cast:binary_update_world (UpdateSyncProto)

player actor sends to game server actor:

- JoinGame (name, variety, color)
- VoteStartGame ()
- Update (DuckProto)
- LeaveGame

game server actor sends to player actor:

- re:JoinGame (id)
- CastJoinGame
- CastLeaveGame
- StartGame (start_time, game_duration)
- UpdateWorld (UpdateSyncProto)
- EndGame ()
