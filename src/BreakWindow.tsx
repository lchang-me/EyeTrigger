import {
  useEffect,
  useRef,
  useState,
} from "react";

import { invoke } from "@tauri-apps/api/core";

import "./BreakWindow.css";


const BREAK_SECONDS = 3 * 60;


function formatCountdown(seconds: number) {
  const minutes =
    Math.floor(seconds / 60);

  const secs =
    seconds % 60;

  return `${minutes}:${secs
    .toString()
    .padStart(2, "0")}`;
}


function BreakWindow() {
  const [running, setRunning] =
    useState(false);

  const [secondsLeft, setSecondsLeft] =
    useState(BREAK_SECONDS);

  const finishing =
    useRef(false);


  const finishBreak = async () => {
    if (finishing.current) {
      return;
    }

    finishing.current = true;

    try {
      await invoke(
        "complete_break"
      );

      await invoke(
        "close_break_window"
      );
    } catch (error) {
      finishing.current = false;

      console.error(
        "Failed to complete break:",
        error
      );
    }
  };


  useEffect(() => {
    if (!running) {
      return;
    }

    if (secondsLeft <= 0) {
      void finishBreak();
      return;
    }

    const timer =
      setTimeout(() => {
        setSecondsLeft(
          (value) => value - 1
        );
      }, 1000);

    return () =>
      clearTimeout(timer);
  }, [
    running,
    secondsLeft,
  ]);


  const startBreak = async () => {
    await invoke("start_break");

    setSecondsLeft(
      BREAK_SECONDS
    );

    setRunning(true);
  };


  const later = async () => {
    await invoke(
      "cancel_break"
    );

    await invoke(
      "close_break_window"
    );
  };


  const progress =
    1 -
    secondsLeft /
      BREAK_SECONDS;


  return (
    <main className="break-shell">
      <header className="break-header">
        <div className="break-brand">
          <span className="break-eye">
            ◉
          </span>

          <span>
            EYE RECOVERY
          </span>
        </div>

        <span className="break-duration">
          3 MIN
        </span>
      </header>


      {!running ? (
        <section className="break-content">
          <div className="break-time-preview">
            3:00
          </div>

          <h1>
            Give your eyes a real break
          </h1>

          <p>
            Step away from the screen,
            relax your focus, and let
            your eye load recover.
          </p>

          <div className="break-actions">
            <button
              className="break-primary"
              onClick={startBreak}
            >
              Start 3 min Break
            </button>

            <button
              className="break-secondary"
              onClick={later}
            >
              Later
            </button>
          </div>
        </section>
      ) : (
        <section className="break-content running">
          <div className="break-orbit">
            <div className="break-countdown">
              {formatCountdown(
                secondsLeft
              )}
            </div>
          </div>

          <h1>
            Look away and relax
          </h1>

          <p>
            Leave the screen alone.
            EyeTrigger will return
            automatically.
          </p>

          <div className="break-progress">
            <div
              className="break-progress-fill"
              style={{
                width:
                  `${progress * 100}%`,
              }}
            />
          </div>
        </section>
      )}


      <footer className="break-footer">
        <span className="footer-dot" />

        EyeTrigger recovery session
      </footer>
    </main>
  );
}


export default BreakWindow;
