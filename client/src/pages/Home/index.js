import { Link } from "@reach/router";
import React from "react";

const Home = ({
  matches,
  user,
  connected,
  onListMatchesClick,
  onCreateMatchClick
}) => (
  <div>
    <h1>{connected ? "Connected" : "Disconnected"}</h1>
    {connected && user && <h3>User: {user.name}</h3>}
    <button disabled={!connected} onClick={onListMatchesClick}>
      List
    </button>
    <button disabled={!connected} onClick={onCreateMatchClick}>
      Create Match
    </button>

    <div className="matches">
      {matches
        .filter(it => it.player1.id !== user.id)
        .map(it => (
          <div>
            <Link to={`match/${it.id}`}>
              <h3>Match with {it.player1.name}</h3>
            </Link>
          </div>
        ))}
    </div>
  </div>
);

export default Home;
