import React, { useEffect, useState } from "react";
import "./App.css";

let ws = new WebSocket("ws://localhost:8000/game/");

const requestMatches = () => {
  ws.send(JSON.stringify({ type: "ListMatches" }));
};

const requestCreateMatch = () => {
  ws.send(JSON.stringify({ type: "CreateMatch" }));
};

function App() {
  let [connected, setConnected] = useState(false);
  const [matches, setMatches] = useState([]);
  useEffect(() => {
    ws.addEventListener("open", function(event) {
      console.log("Connected");
      setConnected(true);
      requestMatches();
    });

    ws.addEventListener("closed", function(event) {
      console.log("Disconnected");
      setConnected(false);
    });

    ws.addEventListener("message", function(event) {
      const response = JSON.parse(event.data);
      const messageMap = {
        ListMatchesResponse: () => {
          setMatches(response.data);
        }
      };
      messageMap[response.type] && messageMap[response.type]();
    });
  }, []);

  const onListMatchesClick = () => {
    requestMatches();
  };

  const onCreateMatchClick = () => {
    requestCreateMatch();
  };
  return (
    <div className="App">
      <h1>{connected ? "Connected" : "Disconnected"}</h1>
      <button disabled={!connected} onClick={onListMatchesClick}>
        List
      </button>
      <button disabled={!connected} onClick={onCreateMatchClick}>
        Create Match
      </button>

      <div className="matches">
        {matches.map(it => (
          <div>
            <a href={it.id}>
              <h3>
                Match: {it.id} User: {it.player1}
              </h3>
            </a>
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;
