const STORAGE_KEY = "visual-dom-builder-state-v2";

const state = {
  nodes: [],
  selectedIds: [],
  gridEnabled: true,
  gridSize: 16,
  canvasSize: { width: 0, height: 0 },
  dirty: false,
  lastSavedAt: null,
  layerAnchorId: null,
};

const refs = {
  canvas: document.querySelector("#editorCanvas"),
  dropHint: document.querySelector("#dropHint"),
  gridToggle: document.querySelector("#gridToggle"),
  gridSize: document.querySelector("#gridSize"),
  saveBtn: document.querySelector("#saveBtn"),
  saveStatus: document.querySelector("#saveStatus"),
  clearBtn: document.querySelector("#clearBtn"),
  nodeCount: document.querySelector("#nodeCount"),
  inspector: document.querySelector("#inspector"),
  emptyInspector: document.querySelector("#emptyInspector"),
  propName: document.querySelector("#propName"),
  propX: document.querySelector("#propX"),
  propY: document.querySelector("#propY"),
  propWidth: document.querySelector("#propWidth"),
  propHeight: document.querySelector("#propHeight"),
  propColor: document.querySelector("#propColor"),
  selectionCount: document.querySelector("#selectionCount"),
  layersTree: document.querySelector("#layersTree"),
  bringFrontBtn: document.querySelector("#bringFrontBtn"),
  moveUpBtn: document.querySelector("#moveUpBtn"),
  moveDownBtn: document.querySelector("#moveDownBtn"),
  sendBackBtn: document.querySelector("#sendBackBtn"),
  nestBtn: document.querySelector("#nestBtn"),
  unnestBtn: document.querySelector("#unnestBtn"),
  htmlCode: document.querySelector("#htmlCode"),
  cssCode: document.querySelector("#cssCode"),
  copyHtml: document.querySelector("#copyHtml"),
  copyCss: document.querySelector("#copyCss"),
  paletteItem: document.querySelector(".palette-item"),
};

let nodeIdCounter = 1;
let activeInteraction = null;
let saveStatusTimer = null;

const resizeHandles = ["n", "e", "s", "w", "ne", "nw", "se", "sw"];

function snap(value) {
  if (!state.gridEnabled) return Math.round(value);
  return Math.round(value / state.gridSize) * state.gridSize;
}

function clamp(value, min, max) {
  return Math.min(Math.max(value, min), max);
}

function readNumber(input, fallback) {
  const value = Number(input.value);
  return Number.isFinite(value) ? value : fallback;
}

function sanitizeLayerName(name, fallback) {
  const cleanName = String(name ?? "").trim();
  return cleanName || fallback;
}

function slugify(text) {
  return (
    String(text)
      .trim()
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, "-")
      .replace(/^-+|-+$/g, "") || "box"
  );
}

function hasTransferType(dataTransfer, type) {
  return Array.from(dataTransfer?.types ?? []).includes(type);
}

function getCanvasSize() {
  const rect = refs.canvas.getBoundingClientRect();
  return {
    width: Math.max(1, Math.round(rect.width)),
    height: Math.max(1, Math.round(rect.height)),
  };
}

function getNode(id) {
  return state.nodes.find((node) => node.id === id) ?? null;
}

function getSelectedNodes() {
  return state.selectedIds.map(getNode).filter(Boolean);
}

function getActiveNode() {
  return getNode(state.selectedIds[state.selectedIds.length - 1]) ?? null;
}

function isSelected(id) {
  return state.selectedIds.includes(id);
}

function getChildren(parentId) {
  return state.nodes.filter((node) => node.parentId === parentId);
}

function getDescendantIds(id) {
  const descendants = [];
  for (const child of getChildren(id)) {
    descendants.push(child.id, ...getDescendantIds(child.id));
  }
  return descendants;
}

function isDescendantOf(id, possibleAncestorId) {
  let node = getNode(id);
  while (node?.parentId) {
    if (node.parentId === possibleAncestorId) return true;
    node = getNode(node.parentId);
  }
  return false;
}

function getNodeGlobalRect(node) {
  let x = node.x;
  let y = node.y;
  let parent = getNode(node.parentId);

  while (parent) {
    x += parent.x;
    y += parent.y;
    parent = getNode(parent.parentId);
  }

  return {
    x,
    y,
    width: node.width,
    height: node.height,
  };
}

function getParentBounds(parentId) {
  const parent = getNode(parentId);
  if (parent) {
    return {
      width: parent.width,
      height: parent.height,
    };
  }

  return getCanvasSize();
}

function getCanvasPoint(event) {
  const rect = refs.canvas.getBoundingClientRect();
  return {
    x: event.clientX - rect.left,
    y: event.clientY - rect.top,
  };
}

function globalToLocalPoint(parentId, point) {
  const parent = getNode(parentId);
  if (!parent) return point;

  const parentRect = getNodeGlobalRect(parent);
  return {
    x: point.x - parentRect.x,
    y: point.y - parentRect.y,
  };
}

function markDirty() {
  state.dirty = true;
  refs.saveStatus.textContent = "Unsaved changes";
}

function normalizeNode(rawNode) {
  const id = String(rawNode.id);
  return {
    id,
    type: "div",
    name: sanitizeLayerName(rawNode.name, id),
    parentId: rawNode.parentId ? String(rawNode.parentId) : null,
    x: Number.isFinite(Number(rawNode.x)) ? Number(rawNode.x) : 0,
    y: Number.isFinite(Number(rawNode.y)) ? Number(rawNode.y) : 0,
    width: Number.isFinite(Number(rawNode.width)) ? Number(rawNode.width) : 240,
    height: Number.isFinite(Number(rawNode.height)) ? Number(rawNode.height) : 160,
    backgroundColor: /^#[0-9a-f]{6}$/i.test(rawNode.backgroundColor)
      ? rawNode.backgroundColor
      : "#2563eb",
    visible: rawNode.visible !== false,
    locked: Boolean(rawNode.locked),
  };
}

function loadDocument() {
  const saved = localStorage.getItem(STORAGE_KEY);
  if (!saved) return;

  try {
    const parsed = JSON.parse(saved);
    const savedNodes = Array.isArray(parsed.nodes) ? parsed.nodes.map(normalizeNode) : [];
    const ids = new Set(savedNodes.map((node) => node.id));

    state.nodes = savedNodes.map((node) => ({
      ...node,
      parentId: ids.has(node.parentId) && node.parentId !== node.id ? node.parentId : null,
    }));
    state.gridEnabled = Boolean(parsed.gridEnabled ?? true);
    state.gridSize = clamp(Number(parsed.gridSize) || 16, 4, 64);
    state.selectedIds = [];
    state.lastSavedAt = parsed.savedAt ?? null;
    state.dirty = false;

    const savedSize = parsed.canvasSize;
    if (savedSize?.width > 0 && savedSize?.height > 0) {
      scaleNodesToCanvasSize(savedSize, getCanvasSize(), false);
    }

    nodeIdCounter =
      state.nodes.reduce((max, node) => {
        const number = Number(node.id.replace("div_", ""));
        return Number.isFinite(number) ? Math.max(max, number + 1) : max;
      }, 1) || 1;

    refs.gridToggle.checked = state.gridEnabled;
    refs.gridSize.value = state.gridSize;
    updateSaveStatus();
  } catch {
    localStorage.removeItem(STORAGE_KEY);
  }
}

function saveDocument(showClick = false) {
  const payload = {
    nodes: state.nodes,
    gridEnabled: state.gridEnabled,
    gridSize: state.gridSize,
    canvasSize: getCanvasSize(),
    savedAt: new Date().toISOString(),
  };

  localStorage.setItem(STORAGE_KEY, JSON.stringify(payload));
  state.lastSavedAt = payload.savedAt;
  state.dirty = false;
  updateSaveStatus();

  if (showClick) {
    refs.saveBtn.classList.remove("is-clicked");
    void refs.saveBtn.offsetWidth;
    refs.saveBtn.classList.add("is-clicked");
  }
}

function updateSaveStatus() {
  window.clearTimeout(saveStatusTimer);

  if (state.dirty) {
    refs.saveStatus.textContent = "Unsaved changes";
    return;
  }

  if (!state.lastSavedAt) {
    refs.saveStatus.textContent = "Not saved";
    return;
  }

  const time = new Date(state.lastSavedAt).toLocaleTimeString([], {
    hour: "2-digit",
    minute: "2-digit",
  });
  refs.saveStatus.textContent = `Saved ${time}`;
}

function createNode(x, y, parentId = null) {
  const width = 240;
  const height = 160;
  const bounds = getParentBounds(parentId);
  const node = {
    id: `div_${nodeIdCounter}`,
    type: "div",
    name: `Div ${nodeIdCounter}`,
    parentId,
    x: clamp(snap(x), 0, Math.max(0, bounds.width - width)),
    y: clamp(snap(y), 0, Math.max(0, bounds.height - height)),
    width: Math.min(width, bounds.width),
    height: Math.min(height, bounds.height),
    backgroundColor: "#2563eb",
    visible: true,
    locked: false,
  };

  nodeIdCounter += 1;
  state.nodes.push(node);
  setSelection([node.id]);
  markDirty();
  render();
}

function classNameForNode(node) {
  const baseName = slugify(node.name || node.id);
  const matchingNodes = state.nodes.filter((item) => slugify(item.name || item.id) === baseName);
  const duplicateIndex = matchingNodes.indexOf(node);
  return duplicateIndex > 0 ? `${baseName}-${duplicateIndex + 1}` : baseName;
}

function indent(level) {
  return "  ".repeat(level);
}

function generateHtmlForParent(parentId, level = 0) {
  return getChildren(parentId)
    .map((node) => {
      const className = classNameForNode(node);
      const children = generateHtmlForParent(node.id, level + 1);

      if (!children) return `${indent(level)}<div class="${className}"></div>`;

      return [
        `${indent(level)}<div class="${className}">`,
        children,
        `${indent(level)}</div>`,
      ].join("\n");
    })
    .join("\n");
}

function generateHtml() {
  return generateHtmlForParent(null);
}

function generateCss() {
  if (state.nodes.length === 0) return "";

  return state.nodes
    .map((node) => {
      const className = classNameForNode(node);
      const hiddenRule = node.visible ? "" : "\n  display: none;";
      return `.${className} {
  position: absolute;
  left: ${node.x}px;
  top: ${node.y}px;
  width: ${node.width}px;
  height: ${node.height}px;
  background-color: ${node.backgroundColor};${hiddenRule}
}`;
    })
    .join("\n\n");
}

function updateNodeElement(node) {
  const element = refs.canvas.querySelector(`[data-node-id="${node.id}"]`);
  if (!element) return;

  element.style.left = `${node.x}px`;
  element.style.top = `${node.y}px`;
  element.style.width = `${node.width}px`;
  element.style.height = `${node.height}px`;
  element.style.backgroundColor = node.backgroundColor;
  element.hidden = !node.visible;
  element.classList.toggle("is-hidden-layer", !node.visible);
  element.classList.toggle("is-locked", node.locked);
  element.setAttribute("aria-label", node.name);
  const tag = element.querySelector(":scope > .node-tag");
  if (tag) tag.textContent = node.name;
}

function updateSelectionClasses() {
  refs.canvas.querySelectorAll(".editable-node").forEach((node) => {
    const nodeId = node.dataset.nodeId;
    node.classList.toggle("is-selected", isSelected(nodeId));
    node.classList.toggle("is-active", nodeId === state.selectedIds[state.selectedIds.length - 1]);
  });

  refs.layersTree.querySelectorAll(".layer-row").forEach((row) => {
    const nodeId = row.dataset.nodeId;
    row.classList.toggle("is-selected", isSelected(nodeId));
    row.classList.toggle("is-active", nodeId === state.selectedIds[state.selectedIds.length - 1]);
  });
}

function updateInspector() {
  const nodes = getSelectedNodes();
  const activeNode = getActiveNode();
  refs.inspector.hidden = nodes.length === 0;
  refs.emptyInspector.hidden = nodes.length > 0;

  if (nodes.length === 0 || !activeNode) {
    refs.emptyInspector.textContent = "No element selected";
    [refs.propX, refs.propY, refs.propWidth, refs.propHeight, refs.propColor].forEach((input) => {
      input.disabled = false;
    });
    return;
  }

  refs.emptyInspector.textContent = "";
  refs.propName.value = activeNode.name;
  refs.propX.value = activeNode.x;
  refs.propY.value = activeNode.y;
  refs.propWidth.value = activeNode.width;
  refs.propHeight.value = activeNode.height;
  refs.propColor.value = activeNode.backgroundColor;

  [refs.propX, refs.propY, refs.propWidth, refs.propHeight, refs.propColor].forEach((input) => {
    input.disabled = activeNode.locked;
  });
}

function updateGeneratedCode() {
  refs.htmlCode.textContent = generateHtml();
  refs.cssCode.textContent = generateCss();
}

function updateMeta() {
  const count = state.nodes.length;
  const selectedCount = state.selectedIds.length;
  refs.nodeCount.textContent = `${count} ${count === 1 ? "node" : "nodes"}`;
  refs.selectionCount.textContent = `${selectedCount} selected`;
  refs.dropHint.hidden = count > 0;
  refs.canvas.classList.toggle("grid-on", state.gridEnabled);
  refs.canvas.style.setProperty("--grid-size", `${state.gridSize}px`);
  refs.bringFrontBtn.disabled = selectedCount === 0;
  refs.moveUpBtn.disabled = selectedCount === 0;
  refs.moveDownBtn.disabled = selectedCount === 0;
  refs.sendBackBtn.disabled = selectedCount === 0;
  refs.nestBtn.disabled = selectedCount < 2 || Boolean(getActiveNode()?.locked);
  refs.unnestBtn.disabled = selectedCount === 0 || !getSelectedNodes().some((node) => node.parentId);
}

function renderNode(node) {
  const element = document.createElement("div");
  element.className = "editable-node";
  element.dataset.nodeId = node.id;
  element.dataset.parentId = node.parentId ?? "";
  element.style.left = `${node.x}px`;
  element.style.top = `${node.y}px`;
  element.style.width = `${node.width}px`;
  element.style.height = `${node.height}px`;
  element.style.backgroundColor = node.backgroundColor;
  element.hidden = !node.visible;
  element.classList.toggle("is-hidden-layer", !node.visible);
  element.classList.toggle("is-locked", node.locked);
  element.setAttribute("role", "button");
  element.setAttribute("tabindex", "0");
  element.setAttribute("aria-label", node.name);

  const tag = document.createElement("span");
  tag.className = "node-tag";
  tag.textContent = node.name;
  element.append(tag);

  for (const handle of resizeHandles) {
    const handleElement = document.createElement("span");
    handleElement.className = `resize-handle handle-${handle}`;
    handleElement.dataset.handle = handle;
    handleElement.setAttribute("aria-hidden", "true");
    element.append(handleElement);
  }

  for (const child of getChildren(node.id)) {
    element.append(renderNode(child));
  }

  return element;
}

function renderLayersForParent(parentId, level = 0) {
  for (const node of [...getChildren(parentId)].reverse()) {
    const row = document.createElement("div");
    row.className = "layer-row";
    row.dataset.nodeId = node.id;
    row.draggable = true;
    row.style.setProperty("--layer-depth", level);
    row.classList.toggle("is-hidden-layer", !node.visible);
    row.classList.toggle("is-locked", node.locked);

    const visibilityButton = document.createElement("button");
    visibilityButton.className = "layer-icon-button layer-visibility-button";
    visibilityButton.type = "button";
    visibilityButton.dataset.layerAction = "visibility";
    visibilityButton.title = node.visible ? "Hide layer" : "Show layer";
    const eyeIcon = document.createElement("span");
    eyeIcon.className = "eye-icon";
    visibilityButton.append(eyeIcon);

    const thumbnail = document.createElement("span");
    thumbnail.className = "layer-thumbnail";
    thumbnail.style.backgroundColor = node.backgroundColor;
    thumbnail.setAttribute("aria-hidden", "true");

    const main = document.createElement("button");
    main.className = "layer-main";
    main.type = "button";
    main.dataset.layerAction = "select";

    const name = document.createElement("span");
    name.className = "layer-name";
    name.textContent = node.name;
    main.append(name);

    const size = document.createElement("span");
    size.className = "layer-size";
    size.textContent = node.parentId ? `child / ${node.width}x${node.height}` : `root / ${node.width}x${node.height}`;
    main.append(size);

    const lockButton = document.createElement("button");
    lockButton.className = "layer-icon-button layer-lock-button";
    lockButton.type = "button";
    lockButton.dataset.layerAction = "lock";
    lockButton.title = node.locked ? "Unlock layer" : "Lock layer";
    const lockIcon = document.createElement("span");
    lockIcon.className = "lock-icon";
    lockButton.append(lockIcon);

    row.append(visibilityButton, thumbnail, main, lockButton);

    refs.layersTree.append(row);
    renderLayersForParent(node.id, level + 1);
  }
}

function renderLayers() {
  refs.layersTree.replaceChildren();

  if (state.nodes.length === 0) {
    const empty = document.createElement("p");
    empty.className = "empty-state layers-empty";
    empty.textContent = "No layers";
    refs.layersTree.append(empty);
    return;
  }

  renderLayersForParent(null);
}

function getFlatLayerOrder(parentId = null) {
  const ids = [];
  for (const node of [...getChildren(parentId)].reverse()) {
    ids.push(node.id, ...getFlatLayerOrder(node.id));
  }
  return ids;
}

function selectLayerFromEvent(id, event) {
  if (event.shiftKey && state.layerAnchorId) {
    const order = getFlatLayerOrder();
    const anchorIndex = order.indexOf(state.layerAnchorId);
    const nextIndex = order.indexOf(id);

    if (anchorIndex >= 0 && nextIndex >= 0) {
      const start = Math.min(anchorIndex, nextIndex);
      const end = Math.max(anchorIndex, nextIndex);
      const anchorId = state.layerAnchorId;
      setSelection(order.slice(start, end + 1));
      state.layerAnchorId = anchorId;
      return;
    }
  }

  if (event.ctrlKey || event.metaKey) {
    toggleSelection(id);
    return;
  }

  setSelection([id]);
}

function render() {
  refs.canvas.querySelectorAll(":scope > .editable-node").forEach((node) => node.remove());
  for (const node of getChildren(null)) {
    refs.canvas.append(renderNode(node));
  }
  renderLayers();
  updateMeta();
  updateSelectionClasses();
  updateInspector();
  updateGeneratedCode();
}

function setSelection(ids) {
  const existing = new Set(state.nodes.map((node) => node.id));
  state.selectedIds = [...new Set(ids)].filter((id) => existing.has(id));
  state.layerAnchorId = state.selectedIds[state.selectedIds.length - 1] ?? null;
  updateSelectionClasses();
  updateInspector();
  updateMeta();
}

function toggleSelection(id) {
  if (isSelected(id)) {
    setSelection(state.selectedIds.filter((selectedId) => selectedId !== id));
  } else {
    setSelection([...state.selectedIds, id]);
  }
}

function applyNodePatch(id, patch, dirty = true) {
  const node = getNode(id);
  if (!node) return;

  Object.assign(node, patch);
  updateNodeElement(node);
  renderLayers();
  updateSelectionClasses();
  updateInspector();
  updateMeta();
  updateGeneratedCode();
  if (dirty) markDirty();
}

function constrainNodeSize(next, parentId) {
  const bounds = getParentBounds(parentId);
  const width = clamp(next.width, 32, Math.max(32, bounds.width));
  const height = clamp(next.height, 32, Math.max(32, bounds.height));
  const x = clamp(next.x, 0, Math.max(0, bounds.width - width));
  const y = clamp(next.y, 0, Math.max(0, bounds.height - height));

  return {
    x: Math.round(x),
    y: Math.round(y),
    width: Math.round(width),
    height: Math.round(height),
  };
}

function getMovableSelection(anchorId) {
  const ids = isSelected(anchorId) ? state.selectedIds : [anchorId];
  return ids.filter((id) => {
    const node = getNode(id);
    return node && !node.locked && !ids.some((selectedId) => selectedId !== id && isDescendantOf(id, selectedId));
  });
}

function startMove(event, nodeElement) {
  const node = getNode(nodeElement.dataset.nodeId);
  if (!node) return;

  if (!isSelected(node.id)) setSelection([node.id]);
  if (node.locked) return;

  const ids = getMovableSelection(node.id);
  if (ids.length === 0) return;
  const point = getCanvasPoint(event);
  activeInteraction = {
    type: "move",
    ids,
    startPoint: point,
    startNodes: ids.map((id) => ({ ...getNode(id) })),
  };
  nodeElement.setPointerCapture(event.pointerId);
  ids.forEach((id) => refs.canvas.querySelector(`[data-node-id="${id}"]`)?.classList.add("is-moving"));
}

function startResize(event, nodeElement, handle) {
  const node = getNode(nodeElement.dataset.nodeId);
  if (!node) return;

  setSelection([node.id]);
  if (node.locked) return;

  activeInteraction = {
    type: "resize",
    id: node.id,
    handle,
    startClientX: event.clientX,
    startClientY: event.clientY,
    startNode: { ...node },
  };
  nodeElement.setPointerCapture(event.pointerId);
  nodeElement.classList.add("is-resizing");
}

function updateMove(event) {
  if (!activeInteraction || activeInteraction.type !== "move") return;

  const point = getCanvasPoint(event);
  const dx = point.x - activeInteraction.startPoint.x;
  const dy = point.y - activeInteraction.startPoint.y;

  for (const start of activeInteraction.startNodes) {
    const next = constrainNodeSize(
      {
        x: snap(start.x + dx),
        y: snap(start.y + dy),
        width: start.width,
        height: start.height,
      },
      start.parentId,
    );
    applyNodePatch(start.id, next);
  }
}

function updateResize(event) {
  if (!activeInteraction || activeInteraction.type !== "resize") return;

  const dx = event.clientX - activeInteraction.startClientX;
  const dy = event.clientY - activeInteraction.startClientY;
  const start = activeInteraction.startNode;
  const handle = activeInteraction.handle;
  let next = {
    x: start.x,
    y: start.y,
    width: start.width,
    height: start.height,
  };

  if (handle.includes("e")) next.width = start.width + dx;
  if (handle.includes("s")) next.height = start.height + dy;
  if (handle.includes("w")) {
    next.x = start.x + dx;
    next.width = start.width - dx;
  }
  if (handle.includes("n")) {
    next.y = start.y + dy;
    next.height = start.height - dy;
  }

  next = {
    x: snap(next.x),
    y: snap(next.y),
    width: snap(next.width),
    height: snap(next.height),
  };

  if (next.width < 32) {
    if (handle.includes("w")) next.x = start.x + start.width - 32;
    next.width = 32;
  }
  if (next.height < 32) {
    if (handle.includes("n")) next.y = start.y + start.height - 32;
    next.height = 32;
  }

  applyNodePatch(activeInteraction.id, constrainNodeSize(next, start.parentId));
}

function finishInteraction(event) {
  if (!activeInteraction) return;

  const ids = activeInteraction.ids ?? [activeInteraction.id];
  for (const id of ids) {
    const nodeElement = refs.canvas.querySelector(`[data-node-id="${id}"]`);
    nodeElement?.classList.remove("is-moving", "is-resizing");
    if (nodeElement?.hasPointerCapture(event.pointerId)) {
      nodeElement.releasePointerCapture(event.pointerId);
    }
  }
  activeInteraction = null;
}

function updateSelectedFromInputs() {
  const node = getActiveNode();
  if (!node) return;

  const nextName = sanitizeLayerName(refs.propName.value, node.id);
  if (node.locked) {
    applyNodePatch(node.id, { name: nextName });
    return;
  }

  const next = constrainNodeSize(
    {
      x: readNumber(refs.propX, node.x),
      y: readNumber(refs.propY, node.y),
      width: readNumber(refs.propWidth, node.width),
      height: readNumber(refs.propHeight, node.height),
    },
    node.parentId,
  );

  applyNodePatch(node.id, {
    ...next,
    name: nextName,
    backgroundColor: refs.propColor.value,
  });
}

function nestSelection() {
  const parent = getActiveNode();
  if (!parent || state.selectedIds.length < 2) return;
  if (parent.locked) return;

  const children = state.selectedIds
    .filter((id) => id !== parent.id)
    .map(getNode)
    .filter((node) => node && !node.locked && !isDescendantOf(parent.id, node.id));

  if (children.length === 0) return;

  const parentRect = getNodeGlobalRect(parent);
  for (const child of children) {
    const childRect = getNodeGlobalRect(child);
    const next = constrainNodeSize(
      {
        x: snap(childRect.x - parentRect.x),
        y: snap(childRect.y - parentRect.y),
        width: child.width,
        height: child.height,
      },
      parent.id,
    );
    Object.assign(child, next, { parentId: parent.id });
  }

  markDirty();
  render();
}

function unnestSelection() {
  const selected = getSelectedNodes().filter((node) => node.parentId && !node.locked);
  if (selected.length === 0) return;

  for (const node of selected) {
    const oldParent = getNode(node.parentId);
    const newParentId = oldParent?.parentId ?? null;
    const globalRect = getNodeGlobalRect(node);
    const localPoint = globalToLocalPoint(newParentId, globalRect);
    const next = constrainNodeSize(
      {
        x: snap(localPoint.x),
        y: snap(localPoint.y),
        width: node.width,
        height: node.height,
      },
      newParentId,
    );
    Object.assign(node, next, { parentId: newParentId });
  }

  markDirty();
  render();
}

function replaceSiblingOrder(parentId, orderedIds) {
  const orderedNodes = orderedIds.map(getNode).filter(Boolean);
  const siblingIndexes = state.nodes
    .map((node, index) => (node.parentId === parentId ? index : -1))
    .filter((index) => index >= 0);

  siblingIndexes.forEach((stateIndex, orderIndex) => {
    state.nodes[stateIndex] = orderedNodes[orderIndex];
  });
}

function reorderSelectedLayers(mode) {
  const selectedNodes = getSelectedNodes().filter((node) => !node.locked);
  if (selectedNodes.length === 0) return;

  const selectedByParent = new Map();
  for (const node of selectedNodes) {
    const key = node.parentId ?? "__root__";
    if (!selectedByParent.has(key)) selectedByParent.set(key, []);
    selectedByParent.get(key).push(node.id);
  }

  for (const [key, selectedIds] of selectedByParent) {
    const parentId = key === "__root__" ? null : key;
    const selectedSet = new Set(selectedIds);
    let order = getChildren(parentId).map((node) => node.id);

    if (mode === "front") {
      order = [
        ...order.filter((id) => !selectedSet.has(id)),
        ...order.filter((id) => selectedSet.has(id)),
      ];
    }

    if (mode === "back") {
      order = [
        ...order.filter((id) => selectedSet.has(id)),
        ...order.filter((id) => !selectedSet.has(id)),
      ];
    }

    if (mode === "up") {
      for (let index = order.length - 2; index >= 0; index -= 1) {
        if (selectedSet.has(order[index]) && !selectedSet.has(order[index + 1])) {
          [order[index], order[index + 1]] = [order[index + 1], order[index]];
        }
      }
    }

    if (mode === "down") {
      for (let index = 1; index < order.length; index += 1) {
        if (selectedSet.has(order[index]) && !selectedSet.has(order[index - 1])) {
          [order[index], order[index - 1]] = [order[index - 1], order[index]];
        }
      }
    }

    replaceSiblingOrder(parentId, order);
  }

  markDirty();
  render();
}

function moveNodeToParentPreservingGlobal(node, parentId) {
  const globalRect = getNodeGlobalRect(node);
  const localPoint = globalToLocalPoint(parentId, globalRect);
  const next = constrainNodeSize(
    {
      x: snap(localPoint.x),
      y: snap(localPoint.y),
      width: node.width,
      height: node.height,
    },
    parentId,
  );
  Object.assign(node, next, { parentId });
}

function getMovableLayerIds(targetParentId, targetId = null) {
  const selectedIds = [...state.selectedIds];
  return selectedIds.filter((id) => {
    const node = getNode(id);
    return (
      node &&
      !node.locked &&
      id !== targetId &&
      targetParentId !== id &&
      !isDescendantOf(targetParentId, id) &&
      !selectedIds.some((selectedId) => selectedId !== id && isDescendantOf(id, selectedId))
    );
  });
}

function arrangeSelectedLayersNear(targetId, placement) {
  const target = getNode(targetId);
  const targetParentId = target?.parentId ?? null;
  const movableIds = getMovableLayerIds(targetParentId, targetId);
  if (movableIds.length === 0) return;

  for (const id of movableIds) {
    const node = getNode(id);
    if (node.parentId !== targetParentId) {
      moveNodeToParentPreservingGlobal(node, targetParentId);
    }
  }

  const selectedSet = new Set(movableIds);
  const orderedMovableIds = getChildren(targetParentId)
    .map((node) => node.id)
    .filter((id) => selectedSet.has(id));

  let order = getChildren(targetParentId)
    .map((node) => node.id)
    .filter((id) => !selectedSet.has(id));

  let insertIndex = order.length;
  if (targetId) {
    const targetIndex = order.indexOf(targetId);
    if (targetIndex < 0) return;
    insertIndex = placement === "above" ? targetIndex + 1 : targetIndex;
  }

  order.splice(insertIndex, 0, ...orderedMovableIds);
  replaceSiblingOrder(targetParentId, order);
  markDirty();
  render();
}

function moveSelectedLayersToParent(parentId) {
  const target = getNode(parentId);
  if (target?.locked) return;
  if (parentId && state.selectedIds.includes(parentId)) return;

  const selectedIds = [...state.selectedIds];
  const movableIds = selectedIds.filter((id) => {
    const node = getNode(id);
    return (
      node &&
      !node.locked &&
      id !== parentId &&
      !isDescendantOf(parentId, id) &&
      !selectedIds.some((selectedId) => selectedId !== id && isDescendantOf(id, selectedId))
    );
  });

  if (movableIds.length === 0) return;

  for (const id of movableIds) {
    const node = getNode(id);
    moveNodeToParentPreservingGlobal(node, parentId);
  }

  markDirty();
  render();
}

function toggleLayerVisibility(id) {
  const node = getNode(id);
  if (!node) return;

  applyNodePatch(id, { visible: !node.visible });
}

function toggleLayerLock(id) {
  const node = getNode(id);
  if (!node) return;

  applyNodePatch(id, { locked: !node.locked });
}

function clearLayerDropIndicators() {
  refs.layersTree.classList.remove("is-root-drop-target");
  refs.layersTree.querySelectorAll(".is-drop-target, .is-drop-above, .is-drop-below").forEach((target) => {
    target.classList.remove("is-drop-target", "is-drop-above", "is-drop-below");
  });
}

function getLayerDropPlacement(event, row) {
  if (!row) return "above";
  const rect = row.getBoundingClientRect();
  return event.clientY < rect.top + rect.height / 2 ? "above" : "below";
}

function markLayerDropTarget(row, placement) {
  clearLayerDropIndicators();
  if (!row) {
    refs.layersTree.classList.add("is-root-drop-target");
    return;
  }

  row.classList.add("is-drop-target", placement === "above" ? "is-drop-above" : "is-drop-below");
}

function deleteSelection() {
  if (state.selectedIds.length === 0) return;

  const deleteIds = new Set();
  for (const id of state.selectedIds) {
    deleteIds.add(id);
    getDescendantIds(id).forEach((descendantId) => deleteIds.add(descendantId));
  }

  state.nodes = state.nodes.filter((node) => !deleteIds.has(node.id));
  state.selectedIds = [];
  markDirty();
  render();
}

function scaleNodesToCanvasSize(fromSize, toSize, dirty = true) {
  const xScale = toSize.width / fromSize.width;
  const yScale = toSize.height / fromSize.height;

  if (!Number.isFinite(xScale) || !Number.isFinite(yScale)) return;
  if (Math.abs(xScale - 1) < 0.001 && Math.abs(yScale - 1) < 0.001) return;

  for (const node of state.nodes) {
    node.x = Math.round(node.x * xScale);
    node.y = Math.round(node.y * yScale);
    node.width = Math.max(32, Math.round(node.width * xScale));
    node.height = Math.max(32, Math.round(node.height * yScale));
  }

  if (dirty) markDirty();
}

function handleCanvasResize() {
  const nextSize = getCanvasSize();
  const previousSize = state.canvasSize;

  if (!previousSize.width || !previousSize.height) {
    state.canvasSize = nextSize;
    return;
  }

  scaleNodesToCanvasSize(previousSize, nextSize);
  state.canvasSize = nextSize;
  render();
}

function fallbackCopy(text) {
  const textarea = document.createElement("textarea");
  textarea.value = text;
  textarea.setAttribute("readonly", "");
  textarea.style.position = "fixed";
  textarea.style.left = "-9999px";
  document.body.append(textarea);
  textarea.select();

  const copied = document.execCommand("copy");
  textarea.remove();
  return copied;
}

async function copyCode(button, code) {
  if (!code) return;
  const originalText = button.textContent;

  try {
    await navigator.clipboard.writeText(code);
    button.textContent = "Copied";
  } catch {
    button.textContent = fallbackCopy(code) ? "Copied" : "Failed";
  }

  window.setTimeout(() => {
    button.textContent = originalText;
  }, 900);
}

refs.paletteItem.addEventListener("dragstart", (event) => {
  event.dataTransfer.setData("component", "div");
  event.dataTransfer.effectAllowed = "copy";
});

refs.paletteItem.addEventListener("click", () => {
  createNode(32 + state.nodes.length * 24, 32 + state.nodes.length * 24);
});

refs.canvas.addEventListener("dragover", (event) => {
  event.preventDefault();
  event.dataTransfer.dropEffect = "copy";
});

refs.canvas.addEventListener("drop", (event) => {
  event.preventDefault();
  if (event.dataTransfer.getData("component") !== "div") return;

  const parentElement = event.target.closest(".editable-node");
  const candidateParentId = parentElement?.dataset.nodeId ?? null;
  const parentId = getNode(candidateParentId)?.locked ? null : candidateParentId;
  const point = globalToLocalPoint(parentId, getCanvasPoint(event));
  createNode(point.x - 120, point.y - 80, parentId);
});

refs.canvas.addEventListener("pointerdown", (event) => {
  const handle = event.target.closest(".resize-handle");
  const nodeElement = event.target.closest(".editable-node");

  if (handle && nodeElement) {
    event.preventDefault();
    startResize(event, nodeElement, handle.dataset.handle);
    return;
  }

  if (nodeElement) {
    event.preventDefault();
    const id = nodeElement.dataset.nodeId;
    if (event.ctrlKey || event.metaKey || event.shiftKey) {
      toggleSelection(id);
      return;
    }
    startMove(event, nodeElement);
    return;
  }

  setSelection([]);
});

refs.canvas.addEventListener("pointermove", (event) => {
  updateMove(event);
  updateResize(event);
});

refs.canvas.addEventListener("pointerup", finishInteraction);
refs.canvas.addEventListener("pointercancel", finishInteraction);

refs.layersTree.addEventListener("click", (event) => {
  const row = event.target.closest(".layer-row");
  if (!row) return;

  const action = event.target.closest("[data-layer-action]")?.dataset.layerAction;
  if (action === "visibility") {
    event.stopPropagation();
    toggleLayerVisibility(row.dataset.nodeId);
    return;
  }

  if (action === "lock") {
    event.stopPropagation();
    toggleLayerLock(row.dataset.nodeId);
    return;
  }

  selectLayerFromEvent(row.dataset.nodeId, event);
});

refs.layersTree.addEventListener("dragstart", (event) => {
  const row = event.target.closest(".layer-row");
  if (!row) return;
  if (event.target.closest(".layer-icon-button")) {
    event.preventDefault();
    return;
  }

  if (!isSelected(row.dataset.nodeId)) setSelection([row.dataset.nodeId]);
  event.dataTransfer.setData("layer-id", row.dataset.nodeId);
  event.dataTransfer.effectAllowed = "move";
});

refs.layersTree.addEventListener("dragover", (event) => {
  if (!hasTransferType(event.dataTransfer, "layer-id")) return;
  event.preventDefault();
  event.dataTransfer.dropEffect = "move";
  const row = event.target.closest(".layer-row");
  markLayerDropTarget(row, getLayerDropPlacement(event, row));
});

refs.layersTree.addEventListener("dragleave", (event) => {
  if (!refs.layersTree.contains(event.relatedTarget)) {
    clearLayerDropIndicators();
  }
});

refs.layersTree.addEventListener("drop", (event) => {
  if (!hasTransferType(event.dataTransfer, "layer-id")) return;
  event.preventDefault();

  const row = event.target.closest(".layer-row");
  const placement = getLayerDropPlacement(event, row);
  clearLayerDropIndicators();
  arrangeSelectedLayersNear(row?.dataset.nodeId ?? null, placement);
});

refs.layersTree.addEventListener("dragend", clearLayerDropIndicators);

refs.gridToggle.addEventListener("change", () => {
  state.gridEnabled = refs.gridToggle.checked;
  updateMeta();
  markDirty();
});

refs.gridSize.addEventListener("input", () => {
  state.gridSize = clamp(readNumber(refs.gridSize, state.gridSize), 4, 64);
  refs.gridSize.value = state.gridSize;
  updateMeta();
  markDirty();
});

refs.saveBtn.addEventListener("click", () => saveDocument(true));

refs.clearBtn.addEventListener("click", () => {
  state.nodes = [];
  state.selectedIds = [];
  markDirty();
  render();
});

refs.nestBtn.addEventListener("click", nestSelection);
refs.unnestBtn.addEventListener("click", unnestSelection);
refs.bringFrontBtn.addEventListener("click", () => reorderSelectedLayers("front"));
refs.moveUpBtn.addEventListener("click", () => reorderSelectedLayers("up"));
refs.moveDownBtn.addEventListener("click", () => reorderSelectedLayers("down"));
refs.sendBackBtn.addEventListener("click", () => reorderSelectedLayers("back"));

[refs.propName, refs.propX, refs.propY, refs.propWidth, refs.propHeight, refs.propColor].forEach((input) => {
  input.addEventListener("input", updateSelectedFromInputs);
});

refs.copyHtml.addEventListener("click", () => copyCode(refs.copyHtml, refs.htmlCode.textContent));
refs.copyCss.addEventListener("click", () => copyCode(refs.copyCss, refs.cssCode.textContent));

document.addEventListener("keydown", (event) => {
  if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "s") {
    event.preventDefault();
    saveDocument(true);
    return;
  }

  const editingField = ["INPUT", "TEXTAREA", "SELECT"].includes(document.activeElement?.tagName);
  if (!editingField && (event.key === "Delete" || event.key === "Backspace")) {
    event.preventDefault();
    deleteSelection();
  }
});

state.canvasSize = getCanvasSize();
loadDocument();
render();

new ResizeObserver(handleCanvasResize).observe(refs.canvas);
