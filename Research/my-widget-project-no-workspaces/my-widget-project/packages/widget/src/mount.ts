import { createButton, createCloseIcon, createSvgIcon } from "./dom";
import {
  normalizeWidgetOptions,
  type NormalizedWidgetOptions,
  type WidgetOptions
} from "./options";
import { createWidgetStyles } from "./styles";

export interface MountedWidget {
  open(): void;
  close(): void;
  toggle(): void;
  destroy(): void;
  isOpen(): boolean;
}

const WIDGET_REGISTRY_KEY = "__AZTRO_WIDGET_REGISTRY__";

type WidgetRegistry = Map<string, MountedWidget>;

declare global {
  interface Window {
    [WIDGET_REGISTRY_KEY]?: WidgetRegistry;
  }
}

function getWidgetRegistry(): WidgetRegistry {
  window[WIDGET_REGISTRY_KEY] ??= new Map<string, MountedWidget>();
  return window[WIDGET_REGISTRY_KEY];
}

function createWidgetId(options: NormalizedWidgetOptions): string {
  return `aztro-widget:${options.appId}`;
}

function createPanelBody(options: NormalizedWidgetOptions): HTMLElement {
  const body = document.createElement("section");
  body.className = "aztro-body";

  const introCard = document.createElement("article");
  introCard.className = "aztro-card";

  const introTitle = document.createElement("h3");
  introTitle.className = "aztro-card-title";
  introTitle.textContent = "Your embedded tool is running";

  const introText = document.createElement("p");
  introText.className = "aztro-card-text";
  introText.textContent =
    "This panel is isolated inside Shadow DOM. Replace this content with your real tool: inspector, command palette, analytics view, support widget, route debugger, onboarding flow, or product-specific assistant.";

  const actionRow = document.createElement("div");
  actionRow.className = "aztro-action-row";

  const primaryAction = createButton("Run widget action", "aztro-action");
  primaryAction.textContent = "Run action";
  primaryAction.addEventListener("click", () => {
    window.dispatchEvent(
      new CustomEvent("aztro-widget:action", {
        detail: {
          appId: options.appId,
          action: "primary",
          timestamp: new Date().toISOString()
        }
      })
    );
  });

  actionRow.append(primaryAction);
  introCard.append(introTitle, introText, actionRow);

  const metaCard = document.createElement("article");
  metaCard.className = "aztro-card";

  const metaTitle = document.createElement("h3");
  metaTitle.className = "aztro-card-title";
  metaTitle.textContent = "Page context";

  const meta = document.createElement("dl");
  meta.className = "aztro-meta";

  const entries: Array<[string, string]> = [
    ["App", options.appId],
    ["Origin", window.location.origin],
    ["Path", window.location.pathname],
    ["Title", document.title || "Untitled"]
  ];

  for (const [label, value] of entries) {
    const term = document.createElement("dt");
    term.textContent = label;

    const description = document.createElement("dd");
    description.textContent = value;

    meta.append(term, description);
  }

  metaCard.append(metaTitle, meta);
  body.append(introCard, metaCard);

  return body;
}

export function mountWidget(rawOptions: WidgetOptions): MountedWidget {
  if (typeof window === "undefined" || typeof document === "undefined") {
    throw new Error("[@aztro/widget] mountWidget must run in a browser.");
  }

  const options = normalizeWidgetOptions(rawOptions);
  const widgetId = createWidgetId(options);
  const registry = getWidgetRegistry();

  const existingWidget = registry.get(widgetId);

  if (existingWidget) {
    return existingWidget;
  }

  let openState = options.initiallyOpen;

  const host = document.createElement("div");
  host.id = widgetId;
  host.dataset.aztroWidgetHost = "true";

  const shadowRoot = host.attachShadow({ mode: "open" });
  shadowRoot.host.setAttribute("data-theme", options.theme);

  const styles = document.createElement("style");
  styles.textContent = createWidgetStyles(options);

  const root = document.createElement("div");
  root.className = "aztro-widget";

  const panel = document.createElement("aside");
  panel.className = "aztro-panel";
  panel.setAttribute("role", "dialog");
  panel.setAttribute("aria-label", options.title);

  const header = document.createElement("header");
  header.className = "aztro-header";

  const titleGroup = document.createElement("div");
  titleGroup.className = "aztro-title-group";

  const title = document.createElement("div");
  title.className = "aztro-title";
  title.textContent = options.title;

  const subtitle = document.createElement("div");
  subtitle.className = "aztro-subtitle";
  subtitle.textContent = options.subtitle;

  titleGroup.append(title, subtitle);

  const closeButton = createButton("Close widget", "aztro-close");
  closeButton.append(createCloseIcon());

  header.append(titleGroup, closeButton);

  const body = createPanelBody(options);
  panel.append(header, body);

  const launcherButton = createButton(`Open ${options.title}`, "aztro-button");
  launcherButton.setAttribute("aria-expanded", String(openState));
  launcherButton.append(createSvgIcon());

  root.append(panel, launcherButton);
  shadowRoot.append(styles, root);

  function syncState(): void {
    panel.dataset.open = String(openState);
    launcherButton.setAttribute("aria-expanded", String(openState));
  }

  function open(): void {
    openState = true;
    syncState();
  }

  function close(): void {
    openState = false;
    syncState();
  }

  function toggle(): void {
    openState = !openState;
    syncState();
  }

  function handleLauncherClick(): void {
    toggle();
  }

  function handleCloseClick(): void {
    close();
  }

  function handleDocumentKeyDown(event: KeyboardEvent): void {
    if (event.key === "Escape" && openState) {
      close();
    }
  }

  function handleDocumentPointerDown(event: PointerEvent): void {
    if (!openState) {
      return;
    }

    const path = event.composedPath();

    if (path.includes(host)) {
      return;
    }

    close();
  }

  launcherButton.addEventListener("click", handleLauncherClick);
  closeButton.addEventListener("click", handleCloseClick);
  document.addEventListener("keydown", handleDocumentKeyDown);
  document.addEventListener("pointerdown", handleDocumentPointerDown);

  document.body.append(host);
  syncState();

  const widget: MountedWidget = {
    open,
    close,
    toggle,
    destroy() {
      launcherButton.removeEventListener("click", handleLauncherClick);
      closeButton.removeEventListener("click", handleCloseClick);
      document.removeEventListener("keydown", handleDocumentKeyDown);
      document.removeEventListener("pointerdown", handleDocumentPointerDown);
      host.remove();
      registry.delete(widgetId);
    },
    isOpen() {
      return openState;
    }
  };

  registry.set(widgetId, widget);

  return widget;
}
