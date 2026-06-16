import type { NormalizedWidgetOptions } from "./options";

export function createWidgetStyles(options: NormalizedWidgetOptions): string {
  const horizontal =
    options.position.endsWith("right") ? "right: 24px;" : "left: 24px;";

  const vertical =
    options.position.startsWith("bottom") ? "bottom: 24px;" : "top: 24px;";

  const panelHorizontal =
    options.position.endsWith("right") ? "right: 0;" : "left: 0;";

  const panelVertical =
    options.position.startsWith("bottom") ? "bottom: 70px;" : "top: 70px;";

  return `
    :host {
      all: initial;
      color-scheme: dark light;
      --aztro-bg: #080a12;
      --aztro-surface: rgba(18, 22, 34, 0.96);
      --aztro-surface-strong: rgba(28, 34, 52, 0.98);
      --aztro-border: rgba(255, 255, 255, 0.14);
      --aztro-border-strong: rgba(255, 255, 255, 0.24);
      --aztro-text: rgba(255, 255, 255, 0.94);
      --aztro-muted: rgba(255, 255, 255, 0.62);
      --aztro-accent: #8b5cf6;
      --aztro-accent-2: #22d3ee;
      --aztro-shadow: 0 28px 90px rgba(0, 0, 0, 0.48);
      --aztro-radius: 18px;
      --aztro-font: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    }

    :host([data-theme="light"]) {
      --aztro-bg: #ffffff;
      --aztro-surface: rgba(255, 255, 255, 0.98);
      --aztro-surface-strong: rgba(244, 247, 252, 0.98);
      --aztro-border: rgba(10, 16, 30, 0.14);
      --aztro-border-strong: rgba(10, 16, 30, 0.24);
      --aztro-text: rgba(10, 16, 30, 0.94);
      --aztro-muted: rgba(10, 16, 30, 0.62);
      --aztro-shadow: 0 28px 90px rgba(10, 16, 30, 0.18);
    }

    @media (prefers-color-scheme: light) {
      :host([data-theme="system"]) {
        --aztro-bg: #ffffff;
        --aztro-surface: rgba(255, 255, 255, 0.98);
        --aztro-surface-strong: rgba(244, 247, 252, 0.98);
        --aztro-border: rgba(10, 16, 30, 0.14);
        --aztro-border-strong: rgba(10, 16, 30, 0.24);
        --aztro-text: rgba(10, 16, 30, 0.94);
        --aztro-muted: rgba(10, 16, 30, 0.62);
        --aztro-shadow: 0 28px 90px rgba(10, 16, 30, 0.18);
      }
    }

    .aztro-widget {
      position: fixed;
      ${horizontal}
      ${vertical}
      z-index: ${options.zIndex};
      font-family: var(--aztro-font);
      pointer-events: none;
    }

    .aztro-widget * {
      box-sizing: border-box;
      font-family: inherit;
    }

    .aztro-button {
      pointer-events: auto;
      width: 54px;
      height: 54px;
      border: 1px solid var(--aztro-border-strong);
      border-radius: 999px;
      background:
        radial-gradient(circle at 30% 20%, rgba(34, 211, 238, 0.34), transparent 34%),
        radial-gradient(circle at 70% 80%, rgba(139, 92, 246, 0.42), transparent 42%),
        var(--aztro-surface);
      color: var(--aztro-text);
      box-shadow: var(--aztro-shadow);
      cursor: pointer;
      display: grid;
      place-items: center;
      transition: transform 150ms ease, border-color 150ms ease, opacity 150ms ease;
      backdrop-filter: blur(18px);
      -webkit-backdrop-filter: blur(18px);
    }

    .aztro-button:hover {
      transform: translateY(-1px) scale(1.025);
      border-color: var(--aztro-accent-2);
    }

    .aztro-button:active {
      transform: translateY(0) scale(0.98);
    }

    .aztro-button:focus-visible {
      outline: 3px solid rgba(34, 211, 238, 0.34);
      outline-offset: 3px;
    }

    .aztro-icon {
      width: 25px;
      height: 25px;
      display: block;
    }

    .aztro-panel {
      pointer-events: auto;
      position: absolute;
      ${panelHorizontal}
      ${panelVertical}
      width: min(${options.width}px, calc(100vw - 48px));
      height: min(${options.height}px, calc(100vh - 120px));
      border: 1px solid var(--aztro-border);
      border-radius: var(--aztro-radius);
      background: var(--aztro-surface);
      color: var(--aztro-text);
      box-shadow: var(--aztro-shadow);
      overflow: hidden;
      display: none;
      backdrop-filter: blur(22px);
      -webkit-backdrop-filter: blur(22px);
    }

    .aztro-panel[data-open="true"] {
      display: flex;
      flex-direction: column;
      animation: aztro-panel-in 140ms ease-out;
    }

    @keyframes aztro-panel-in {
      from { opacity: 0; transform: translateY(8px) scale(0.985); }
      to { opacity: 1; transform: translateY(0) scale(1); }
    }

    .aztro-header {
      min-height: 58px;
      padding: 0 14px 0 16px;
      border-bottom: 1px solid var(--aztro-border);
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 12px;
      background: linear-gradient(180deg, var(--aztro-surface-strong), transparent);
    }

    .aztro-title-group {
      min-width: 0;
      display: grid;
      gap: 3px;
    }

    .aztro-title {
      font-size: 14px;
      font-weight: 700;
      line-height: 1.2;
      color: var(--aztro-text);
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }

    .aztro-subtitle {
      font-size: 11px;
      line-height: 1.2;
      color: var(--aztro-muted);
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }

    .aztro-close {
      width: 34px;
      height: 34px;
      border: 1px solid transparent;
      border-radius: 10px;
      background: transparent;
      color: var(--aztro-muted);
      cursor: pointer;
      display: grid;
      place-items: center;
      transition: color 130ms ease, background 130ms ease, border-color 130ms ease;
    }

    .aztro-close:hover {
      color: var(--aztro-text);
      background: var(--aztro-surface-strong);
      border-color: var(--aztro-border);
    }

    .aztro-body {
      flex: 1;
      min-height: 0;
      overflow: auto;
      padding: 16px;
      display: grid;
      align-content: start;
      gap: 14px;
    }

    .aztro-card {
      border: 1px solid var(--aztro-border);
      border-radius: 14px;
      background: var(--aztro-surface-strong);
      padding: 14px;
      display: grid;
      gap: 8px;
    }

    .aztro-card-title {
      margin: 0;
      color: var(--aztro-text);
      font-size: 13px;
      font-weight: 700;
      line-height: 1.3;
    }

    .aztro-card-text {
      margin: 0;
      color: var(--aztro-muted);
      font-size: 12px;
      line-height: 1.55;
    }

    .aztro-action-row {
      display: flex;
      flex-wrap: wrap;
      gap: 8px;
      margin-top: 4px;
    }

    .aztro-action {
      height: 36px;
      border: 1px solid var(--aztro-border-strong);
      border-radius: 12px;
      background: linear-gradient(135deg, rgba(139, 92, 246, 0.28), rgba(34, 211, 238, 0.16));
      color: var(--aztro-text);
      font-size: 12px;
      font-weight: 700;
      cursor: pointer;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      padding: 0 12px;
    }

    .aztro-action:hover {
      border-color: var(--aztro-accent-2);
    }

    .aztro-meta {
      display: grid;
      grid-template-columns: 80px minmax(0, 1fr);
      gap: 8px 10px;
      font-size: 12px;
      line-height: 1.45;
    }

    .aztro-meta dt {
      margin: 0;
      color: var(--aztro-muted);
    }

    .aztro-meta dd {
      margin: 0;
      color: var(--aztro-text);
      overflow-wrap: anywhere;
    }

    @media (max-width: 640px) {
      .aztro-widget {
        right: 16px;
        left: auto;
        bottom: 16px;
        top: auto;
      }

      .aztro-panel {
        right: 0;
        left: auto;
        bottom: 70px;
        top: auto;
        width: calc(100vw - 32px);
        height: min(${options.height}px, calc(100vh - 104px));
      }
    }
  `;
}
