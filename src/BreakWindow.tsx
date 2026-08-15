import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import "./BreakWindow.css";

const BREAK_SECONDS = 3 * 60;

function formatCountdown(seconds: number) {
  const minutes = Math.floor(seconds / 60);
  const secs = seconds % 60;

  return `${minutes}:${secs
    .toString()
    .padStart(2, "0")}`;
}

function BreakWindow() {
  const [running, setRunning] =
    useState(false);

  const [secondsLeft, setSecondsLeft] =
    useState(BREAK_SECONDS);

  const [finishing, setFinishing] =
    useState(false);


  const finishBreak = async () => {
    if (finishing) {
      return;
    }

    setFinishing(true);

    try {
      await invoke("complete_break");

      await invoke(
        "close_break_window"
      );
    } catch (error) {
      console.error(
        "Failed to complete break:",
        error
      );

      setFinishing(false);
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

    const timer = setTimeout(() => {
      setSecondsLeft(
        (value) => value - 1
      );
    }, 1000);

    return () => {
      clearTimeout(timer);
    };
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
    // 这里只表示：
    // “我现在不休息”
    //
    // 不取消 Level 2 overdue 状态。
    // 后面达到 Level 3 时必须强制休息。
    await invoke("cancel_break");

    await invoke(
      "close_break_window"
    );
  };


  return (
    <main className="break-window">
      {!running ? (
        <>
          <h1>Time for a break</h1>

          <p>
            Step away from the screen
            for 3 minutes.
          </p>

          <button
            onClick={startBreak}
          >
            Start 3 min Break
          </button>

          <button
            onClick={later}
          >
            Later
          </button>
        </>
      ) : (
        <>
          <h1>
            Look away and relax
          </h1>

          <div className="countdown">
            {formatCountdown(
              secondsLeft
            )}
          </div>

          <p>
            Stay away from the screen
            until the break is complete.
          </p>
        </>
      )}
    </main>
  );
}

export default BreakWindow;
