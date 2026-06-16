import { mountWidget } from "@aztro/widget";

import "./page.css";

const widget = mountWidget({
  appId: "demo-app",
  title: "AZTRO Tool",
  subtitle: "Demo embedded widget",
  position: "bottom-right",
  theme: "system",
  initiallyOpen: false,
  width: 440,
  height: 600
});

window.addEventListener("aztro-widget:action", (event) => {
  if (!(event instanceof CustomEvent)) {
    return;
  }

  console.log("Widget action received:", event.detail);
});

window.addEventListener("beforeunload", () => {
  widget.destroy();
});
