export function createSvgIcon(): SVGElement {
  const namespace = "http://www.w3.org/2000/svg";

  const svg = document.createElementNS(namespace, "svg");
  svg.setAttribute("class", "aztro-icon");
  svg.setAttribute("viewBox", "0 0 24 24");
  svg.setAttribute("fill", "none");
  svg.setAttribute("aria-hidden", "true");

  const pathA = document.createElementNS(namespace, "path");
  pathA.setAttribute(
    "d",
    "M4.75 12.25C7.55 6.5 10.7 4.25 13.95 5.4C17.2 6.55 18.85 9.55 18.9 14.25"
  );
  pathA.setAttribute("stroke", "currentColor");
  pathA.setAttribute("stroke-width", "1.8");
  pathA.setAttribute("stroke-linecap", "round");

  const pathB = document.createElementNS(namespace, "path");
  pathB.setAttribute(
    "d",
    "M19.25 11.75C16.45 17.5 13.3 19.75 10.05 18.6C6.8 17.45 5.15 14.45 5.1 9.75"
  );
  pathB.setAttribute("stroke", "currentColor");
  pathB.setAttribute("stroke-width", "1.8");
  pathB.setAttribute("stroke-linecap", "round");

  const dot = document.createElementNS(namespace, "circle");
  dot.setAttribute("cx", "12");
  dot.setAttribute("cy", "12");
  dot.setAttribute("r", "1.65");
  dot.setAttribute("fill", "currentColor");

  svg.append(pathA, pathB, dot);

  return svg;
}

export function createCloseIcon(): SVGElement {
  const namespace = "http://www.w3.org/2000/svg";

  const svg = document.createElementNS(namespace, "svg");
  svg.setAttribute("viewBox", "0 0 24 24");
  svg.setAttribute("fill", "none");
  svg.setAttribute("width", "18");
  svg.setAttribute("height", "18");
  svg.setAttribute("aria-hidden", "true");

  const path = document.createElementNS(namespace, "path");
  path.setAttribute("d", "M7 7L17 17M17 7L7 17");
  path.setAttribute("stroke", "currentColor");
  path.setAttribute("stroke-width", "2");
  path.setAttribute("stroke-linecap", "round");

  svg.append(path);

  return svg;
}

export function createButton(
  ariaLabel: string,
  className: string
): HTMLButtonElement {
  const button = document.createElement("button");
  button.type = "button";
  button.className = className;
  button.setAttribute("aria-label", ariaLabel);

  return button;
}
