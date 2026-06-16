export type WidgetPosition =
  | "bottom-right"
  | "bottom-left"
  | "top-right"
  | "top-left";

export type WidgetTheme = "dark" | "light" | "system";

export interface WidgetOptions {
  appId: string;
  title?: string;
  subtitle?: string;
  position?: WidgetPosition;
  theme?: WidgetTheme;
  initiallyOpen?: boolean;
  zIndex?: number;
  width?: number;
  height?: number;
}

export interface NormalizedWidgetOptions {
  appId: string;
  title: string;
  subtitle: string;
  position: WidgetPosition;
  theme: WidgetTheme;
  initiallyOpen: boolean;
  zIndex: number;
  width: number;
  height: number;
}

export function normalizeWidgetOptions(
  options: WidgetOptions
): NormalizedWidgetOptions {
  const appId = options.appId.trim();

  if (appId.length === 0) {
    throw new Error("[@aztro/widget] appId is required.");
  }

  return {
    appId,
    title: options.title ?? "AZTRO Tool",
    subtitle: options.subtitle ?? "Embedded widget",
    position: options.position ?? "bottom-right",
    theme: options.theme ?? "system",
    initiallyOpen: options.initiallyOpen ?? false,
    zIndex: options.zIndex ?? 2147483000,
    width: options.width ?? 420,
    height: options.height ?? 560
  };
}
