import { useEffect, useState } from "react";
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


// ============================================================
// Main EyeTrigger window
//
// 这里只显示 Rust 后台状态。
// 不再负责 reminder dispatch。
// ============================================================

function MainApp() {
  const [status, setStatus] =
    useState<EyeStatus>({
      idle_seconds: 0,
      session_seconds: 0,
      fatigue: 0,
      reminder_level: 0,
      active: true,
    });


  // ----------------------------------------------------------
  // 每秒读取一次 Rust 状态
  // ----------------------------------------------------------

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


    // 打开窗口后立即更新一次
    update();


    const timer =
      setInterval(
        update,
        1000
      );


    return () => {
      clearInterval(timer);
    };
  }, []);


  const fatiguePercent =
    Math.round(
      status.fatigue * 100
    );


  return (
    <main className="container">
      <h1>
        EyeTrigger
      </h1>

      <h2>
        {status.active
          ? "Active"
          : "Away"}
      </h2>


      <p>
        Session:{" "}
        <strong>
          {formatTime(
            status.session_seconds
          )}
        </strong>
      </p>


      <p>
        Eye Load:{" "}
        <strong>
          {fatiguePercent}%
        </strong>
      </p>


      <progress
        value={fatiguePercent}
        max="100"
      />


      <p>
        Reminder Level:{" "}
        <strong>
          {status.reminder_level}
        </strong>
      </p>


      <p>
        Idle:{" "}
        {status.idle_seconds.toFixed(1)} s
      </p>
    </main>
  );
}


// ============================================================
// Window router
//
// Rust 创建不同 WebviewWindow 时，通过 URL 参数决定
// 当前窗口应该渲染哪个 React component。
// ============================================================

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
