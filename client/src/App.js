import { Router } from "@reach/router";
import React, { useEffect, useState } from "react";
import "./App.css";
import Home from "./pages/Home";
import Match from "./pages/Match";

let ws = new WebSocket("ws://localhost:8000/game/");

const requestMatches = () => {
  ws.send(JSON.stringify({ type: "ListMatches" }));
};

const requestCreateMatch = () => {
  ws.send(JSON.stringify({ type: "CreateMatch" }));
};

function App() {
  const [connected, setConnected] = useState(false);
  const [user, setUser] = useState(null);
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
      const data = response.data;
      const messageMap = {
        LoginResponse: () => setUser(data),
        ListMatchesResponse: () => setMatches(response.data)
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
      <Router>
        <Home
          path="/"
          connected={connected}
          user={user}
          matches={matches}
          onCreateMatchClick={onCreateMatchClick}
          onListMatchesClick={onListMatchesClick}
        />
        <Match path="/match/:matchId" />
      </Router>
    </div>
  );
}

export default App;
