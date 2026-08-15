import {
  useEffect,
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
  const [secondsLeft, setSecondsLeft] =
    useState(
      STRONG_BREAK_SECONDS
    );


  // 一进入 Level 3，
  // 立刻开始正式休息状态。
  useEffect(() => {
    void invoke("start_break");
  }, []);


  useEffect(() => {
    if (secondsLeft <= 0) {
      const finish =
        async () => {
          await invoke(
            "complete_strong_break"
          );

          await invoke(
            "close_dim_window"
          );
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


  return (
    <main className="dim-window">
      <h1>
        Time to stop
      </h1>

      <p>
        You skipped your break.
      </p>

      <p>
        Please step away from
        the screen.
      </p>

      <div className="dim-countdown">
        {formatCountdown(
          secondsLeft
        )}
      </div>
    </main>
  );
}


export default DimWindow;
