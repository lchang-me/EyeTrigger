import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import "./GentleWindow.css";


function GentleWindow() {
  const [processing, setProcessing] =
    useState(false);


  const completeBreak = async () => {
    // 防止一次提醒被连续点击多次
    if (processing) {
      return;
    }

    setProcessing(true);

    try {
      // 用户确认已经完成约20秒远眺
      //
      // Rust:
      // 1. Eye Load下降
      // 2. acknowledge gentle
      await invoke(
        "complete_gentle_break"
      );


      // 隐藏 NSPanel
      await invoke(
        "close_gentle_window"
      );


      // 非常重要：
      //
      // NSPanel只是hide，并没有销毁React。
      // 所以必须把状态恢复，
      // 否则下一次show时processing仍然是true。
      setProcessing(false);

    } catch (error) {
      console.error(
        "Failed to complete gentle break:",
        error
      );

      setProcessing(false);
    }
  };


  return (
    <main
      className="gentle-window"
      onClick={completeBreak}
      title="Click after looking away for about 20 seconds"
    >
      <span className="eye-icon">
        ◉
      </span>

      <div className="gentle-text">
        <strong>
          Look away for about 20 seconds
        </strong>

        <span>
          {processing
            ? "Done"
            : "Click here when you're done"}
        </span>
      </div>
    </main>
  );
}


export default GentleWindow;
