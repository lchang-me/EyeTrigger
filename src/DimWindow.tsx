import {
  useEffect,
  useRef,
  useState,
} from "react";

import { invoke } from "@tauri-apps/api/core";

import "./DimWindow.css";


const STRONG_BREAK_SECONDS =
  5 * 60;


function formatCountdown(
  seconds: number
) {
  const minutes =
    Math.floor(seconds / 60);

  const secs =
    seconds % 60;

  return `${minutes}:${secs
    .toString()
    .padStart(2, "0")}`;
}


function DimWindow() {
  const [
    secondsLeft,
    setSecondsLeft,
  ] = useState(
    STRONG_BREAK_SECONDS
  );

  const finishing =
    useRef(false);


  useEffect(() => {
    void invoke("start_break");
  }, []);


  useEffect(() => {
    if (secondsLeft <= 0) {
      if (finishing.current) {
        return;
      }

      finishing.current = true;

      const finish =
        async () => {
          try {
            await invoke(
              "complete_strong_break"
            );

            await invoke(
              "close_dim_window"
            );
          } catch (error) {
            finishing.current =
              false;

            console.error(
              "Failed to finish strong break:",
              error
            );
          }
        };

      void finish();

      return;
    }


    const timer =
      setTimeout(() => {
        setSecondsLeft(
          (value) =>
            value - 1
        );
      }, 1000);


    return () =>
      clearTimeout(timer);
  }, [secondsLeft]);


  const progress =
    1 -
    secondsLeft /
      STRONG_BREAK_SECONDS;


  return (
    <main className="strong-shell">
      <div className="strong-glow" />


      <header className="strong-header">
        <span className="strong-logo">
          ◉
        </span>

        <span>
          EyeTrigger
        </span>
      </header>


      <section className="strong-content">
        <span className="strong-label">
          RECOVERY REQUIRED
        </span>


        <h1>
          Time to step away.
        </h1>


        <p>
          Your eye load continued to
          rise after the break reminder.
          Give your eyes time to recover.
        </p>


        <div className="strong-countdown">
          {formatCountdown(
            secondsLeft
          )}
        </div>


        <span className="strong-caption">
          No screen. Look farther away,
          move around, and relax.
        </span>


        <div className="strong-progress">
          <div
            className="strong-progress-fill"
            style={{
              width:
                `${progress * 100}%`,
            }}
          />
        </div>
      </section>


      <footer className="strong-footer">
        EyeTrigger will return automatically
        when recovery is complete.
      </footer>
    </main>
  );
}


export default DimWindow;
