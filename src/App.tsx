import {
  type CSSProperties,
  useEffect,
  useState,
} from "react";

import { invoke } from "@tauri-apps/api/core";

import BreakWindow from "./BreakWindow";
import GentleWindow from "./GentleWindow";
import DimWindow from "./DimWindow";

import "./App.css";


type EyeStatus = {
  idle_seconds: number;
  session_seconds: number;
  fatigue: number;
  reminder_level: number;
  active: boolean;
};


function formatTime(seconds: number) {
  const total = Math.floor(seconds);

  const minutes =
    Math.floor(total / 60);

  const secs =
    total % 60;

  return `${minutes}m ${secs}s`;
}


function getLoadState(percent: number) {
  if (percent >= 86) {
    return {
      tone: "critical",
      title: "Break required",
      description:
        "You've been working for a while.",
    };
  }

  if (percent >= 74) {
    return {
      tone: "high",
      title: "Time for a real break",
      description:
        "Step away and let your eyes recover.",
    };
  }

  if (percent >= 50) {
    return {
      tone: "medium",
      title: "Building eye load",
      description:
        "A short look away will help.",
    };
  }

  return {
    tone: "low",
    title: "Comfortable",
    description:
      "Your eye load is in a good range.",
  };
}


function MainApp() {
  const [status, setStatus] =
    useState<EyeStatus>({
      idle_seconds: 0,
      session_seconds: 0,
      fatigue: 0,
      reminder_level: 0,
      active: true,
    });


  useEffect(() => {
    const update = async () => {
      try {
        const result =
          await invoke<EyeStatus>(
            "get_eye_status"
          );

        setStatus(result);
      } catch (error) {
        console.error(
          "Failed to read EyeTrigger status:",
          error
        );
      }
    };

    void update();

    const timer =
      setInterval(
        () => void update(),
        1000
      );

    return () =>
      clearInterval(timer);
  }, []);


  const fatiguePercent =
    Math.round(
      status.fatigue * 100
    );

  const loadState =
    getLoadState(
      fatiguePercent
    );

  const ringStyle = {
    "--progress":
      `${fatiguePercent * 3.6}deg`,
  } as CSSProperties;


  return (
    <main
      className="app-shell"
      data-tone={loadState.tone}
    >
      <header className="app-header">
        <div className="brand">
          <div className="brand-icon">
            ◉
          </div>

          <div>
            <h1>
              EyeTrigger
            </h1>

            <span>
              Eye comfort monitor
            </span>
          </div>
        </div>

        <div
          className={
            status.active
              ? "status-pill active"
              : "status-pill away"
          }
        >
          <span className="status-dot" />

          {status.active
            ? "Active"
            : "Away"}
        </div>
      </header>


      <section className="load-section">
        <div
          className="load-ring"
          style={ringStyle}
        >
          <div className="load-ring-inner">
            <strong>
              {fatiguePercent}
              <span>%</span>
            </strong>

            <small>
              EYE LOAD
            </small>
          </div>
        </div>


        <div className="load-message">
          <h2>
            {loadState.title}
          </h2>

          <p>
            {loadState.description}
          </p>
        </div>
      </section>


      <section className="metrics">
        <div className="metric-card">
          <span>
            SESSION
          </span>

          <strong>
            {formatTime(
              status.session_seconds
            )}
          </strong>
        </div>

        <div className="metric-card">
          <span>
            IDLE
          </span>

          <strong>
            {status.idle_seconds < 60
              ? `${Math.floor(
                  status.idle_seconds
                )}s`
              : formatTime(
                  status.idle_seconds
                )}
          </strong>
        </div>
      </section>


      <footer className="app-footer">
        <span className="footer-eye">
          ◉
        </span>

        Running quietly in your menu bar
      </footer>
    </main>
  );
}


function App() {
  const params =
    new URLSearchParams(
      window.location.search
    );

  const windowType =
    params.get("window");


  if (windowType === "gentle") {
    return <GentleWindow />;
  }

  if (windowType === "break") {
    return <BreakWindow />;
  }

  if (windowType === "dim") {
    return <DimWindow />;
  }

  return <MainApp />;
}


export default App;
