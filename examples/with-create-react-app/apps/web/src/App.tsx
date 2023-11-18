import React from "react";
import { Link } from "ui";
import "./App.css";

function App(): JSX.Element {
  return (
    <div className="App">
      <header className="App-header">
        <h1 className="header">
          Web
          <div className="Nxpkgrepo">Nxpkgrepo Example</div>
        </h1>
        <div>
          <Link className="App-link" href="https://nxpkg.build/repo">
            Nxpkgrepo Docs
          </Link>
          <span> | </span>
          <Link className="App-link" href="https://reactjs.org">
            React Docs
          </Link>
        </div>
      </header>
    </div>
  );
}

export default App;
