import { researchWidgetEventClient } from './researchWidgetEventClient'

export type BuilderElement = {
  id: string
  name: string
  kind: 'div' | 'img'
  parentId: string | null
  x: number
  y: number
  width: number
  height: number
  color: string
  visible: boolean
  src?: string
  alt?: string
}

export type BuilderGrid = {
  enabled: boolean
  size: number
}

export type ResearchWidgetState = {
  elements: Array<BuilderElement>
  selectedElementId: string | null
  selectedElementIds: Array<string>
  grid: BuilderGrid
  lastSavedAt: string | null
  saveCount: number
  history: Array<BuilderHistoryEntry>
}

export type BuilderHistoryEntry = {
  id: number
  action: string
  at: string
  snapshot: {
    elements: Array<BuilderElement>
    selectedElementIds: Array<string>
    grid: BuilderGrid
  }
}

type Listener = () => void

const defaultElementSize = {
  width: 160,
  height: 112,
}

const elementColors = ['#4f8cff', '#2f7148', '#d06f45', '#8554d6', '#c43d5a']
const storageKey = 'component-builder-widget-state'

function createInitialState(): ResearchWidgetState {
  return {
    elements: [],
    selectedElementId: null,
    selectedElementIds: [],
    grid: {
      enabled: true,
      size: 16,
    },
    lastSavedAt: null,
    saveCount: 0,
    history: [],
  }
}

class ResearchWidgetStore {
  private state = loadSavedState()
  private listeners = new Set<Listener>()
  private nextElementNumber = getNextElementNumber(this.state.elements)
  private nextHistoryId = 1

  getSnapshot = () => this.state

  subscribe = (listener: Listener) => {
    this.listeners.add(listener)
    return () => {
      this.listeners.delete(listener)
    }
  }

  addDiv = () => {
    const element = this.createElement('div', {
      parentId: null,
      x: 80 + this.state.elements.length * 24,
      y: 72 + this.state.elements.length * 24,
    })

    this.commit(`added ${element.name}`, (state) => ({
      ...state,
      elements: [...state.elements, element],
      selectedElementId: element.id,
      selectedElementIds: [element.id],
    }))
  }

  placeDiv = (x: number, y: number) => {
    const element = this.createElement('div', { parentId: null, x, y })

    this.commit(`placed ${element.name}`, (state) => ({
      ...state,
      elements: [...state.elements, element],
      selectedElementId: element.id,
      selectedElementIds: [element.id],
    }))
  }

  addImageToParent = (
    parentId: string,
    image: { src: string; alt: string; x: number; y: number },
  ) => {
    const parent = this.state.elements.find((element) => element.id === parentId)
    if (!parent || parent.kind !== 'div') return

    const element = this.createElement('img', {
      parentId,
      x: Math.max(0, Math.min(image.x, parent.width - 48)),
      y: Math.max(0, Math.min(image.y, parent.height - 48)),
      src: image.src,
      alt: image.alt,
      width: Math.min(140, parent.width),
      height: Math.min(100, parent.height),
    })

    this.commit(`added ${element.name}`, (state) => ({
      ...state,
      elements: [...state.elements, element],
      selectedElementId: element.id,
      selectedElementIds: [element.id],
    }))
  }

  duplicateSelectedElements = () => {
    if (this.state.selectedElementIds.length === 0) return

    const selectedIdSet = new Set(this.state.selectedElementIds)
    const selectedRoots = this.state.selectedElementIds.filter(
      (elementId) =>
        !hasSelectedAncestor(elementId, selectedIdSet, this.state.elements),
    )
    const cloneMap = new Map<string, string>()
    const clonedElements: Array<BuilderElement> = []

    selectedRoots.forEach((elementId) => {
      const element = this.state.elements.find(
        (candidate) => candidate.id === elementId,
      )
      if (!element) return

      this.cloneElementTree(element, null, cloneMap, clonedElements)
    })

    if (clonedElements.length === 0) return

    const clonedRootIds = selectedRoots
      .map((elementId) => cloneMap.get(elementId))
      .filter((elementId): elementId is string => Boolean(elementId))

    this.commit('duplicated selected elements', (state) => ({
      ...state,
      elements: [...state.elements, ...clonedElements],
      selectedElementId: clonedRootIds[clonedRootIds.length - 1] ?? null,
      selectedElementIds: clonedRootIds,
    }))
  }

  selectElement = (elementId: string, additive = false) => {
    if (!this.state.elements.some((element) => element.id === elementId)) return

    this.commit('selected div element', (state) => {
      if (!additive) {
        return {
          ...state,
          selectedElementId: elementId,
          selectedElementIds: [elementId],
        }
      }

      const isSelected = state.selectedElementIds.includes(elementId)
      const nextSelectedElementIds = isSelected
        ? state.selectedElementIds.filter((id) => id !== elementId)
        : [...state.selectedElementIds, elementId]

      return {
        ...state,
        selectedElementId:
          nextSelectedElementIds[nextSelectedElementIds.length - 1] ?? null,
        selectedElementIds: nextSelectedElementIds,
      }
    })
  }

  nestSelectedElements = () => {
    const [parentId, ...childIds] = this.state.selectedElementIds
    if (!parentId || childIds.length === 0) return

    const parent = this.state.elements.find((element) => element.id === parentId)
    if (!parent) return

    const parentAbsolutePosition = getAbsolutePosition(parent, this.state.elements)
    const childIdSet = new Set(childIds)

    this.commit('nested selected elements', (state) => ({
      ...state,
      elements: state.elements.map((element) => {
        if (!childIdSet.has(element.id) || element.id === parentId) return element
        if (isAncestor(element.id, parentId, state.elements)) return element

        const absolutePosition = getAbsolutePosition(element, state.elements)

        return {
          ...element,
          parentId,
          x: absolutePosition.x - parentAbsolutePosition.x,
          y: absolutePosition.y - parentAbsolutePosition.y,
        }
      }),
      selectedElementId: parentId,
      selectedElementIds: [parentId, ...childIds],
    }))
  }

  unnestSelectedElements = () => {
    if (this.state.selectedElementIds.length === 0) return

    const selectedIdSet = new Set(this.state.selectedElementIds)

    this.commit('unnested selected elements', (state) => ({
      ...state,
      elements: state.elements.map((element) => {
        if (!selectedIdSet.has(element.id) || !element.parentId) return element

        const absolutePosition = getAbsolutePosition(element, state.elements)

        return {
          ...element,
          parentId: null,
          x: absolutePosition.x,
          y: absolutePosition.y,
        }
      }),
    }))
  }

  toggleSelectedVisibility = () => {
    if (this.state.selectedElementIds.length === 0) return

    const selectedIdSet = new Set(this.state.selectedElementIds)
    const shouldShow = this.state.elements.some(
      (element) => selectedIdSet.has(element.id) && !element.visible,
    )

    this.commit('toggled selected visibility', (state) => ({
      ...state,
      elements: state.elements.map((element) =>
        selectedIdSet.has(element.id)
          ? { ...element, visible: shouldShow }
          : element,
      ),
    }))
  }

  moveSelectedLayers = (direction: 'up' | 'down') => {
    if (this.state.selectedElementIds.length === 0) return

    const selectedIdSet = new Set(this.state.selectedElementIds)

    this.commit(`moved selected layers ${direction}`, (state) => ({
      ...state,
      elements: reorderElementsWithinParents(
        state.elements,
        selectedIdSet,
        direction,
      ),
    }))
  }

  updateSelectedElement = (updates: Partial<BuilderElement>) => {
    const selectedElement = this.getSelectedElement()
    if (!selectedElement) return

    this.commit(`updated ${selectedElement.name}`, (state) => ({
      ...state,
      elements: state.elements.map((element) =>
        element.id === selectedElement.id ? { ...element, ...updates } : element,
      ),
    }))
  }

  updateElement = (elementId: string, updates: Partial<BuilderElement>) => {
    const targetElement = this.state.elements.find(
      (element) => element.id === elementId,
    )
    if (!targetElement) return

    this.commit(`updated ${targetElement.name}`, (state) => ({
      ...state,
      elements: state.elements.map((element) =>
        element.id === elementId ? { ...element, ...updates } : element,
      ),
      selectedElementId: elementId,
      selectedElementIds: state.selectedElementIds.includes(elementId)
        ? state.selectedElementIds
        : [elementId],
    }))
  }

  toggleGrid = () => {
    this.commit('toggled grid mode', (state) => ({
      ...state,
      grid: {
        ...state.grid,
        enabled: !state.grid.enabled,
      },
    }))
  }

  setGridSize = (size: number) => {
    this.commit('changed grid size', (state) => ({
      ...state,
      grid: {
        ...state.grid,
        size: Math.max(4, Math.min(64, size)),
      },
    }))
  }

  save = () => {
    const savedAt = new Intl.DateTimeFormat(undefined, {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    }).format(new Date())

    saveState(this.state)
    this.state = {
      ...this.state,
      lastSavedAt: savedAt,
      saveCount: this.state.saveCount + 1,
    }
    this.emit()
  }

  reset = () => {
    this.nextElementNumber = 1
    this.nextHistoryId = 1
    this.state = createInitialState()
    clearSavedState()
    this.emit()
  }

  private createElement(
    kind: BuilderElement['kind'],
    position: {
    parentId: string | null
    x: number
    y: number
    width?: number
    height?: number
    src?: string
    alt?: string
  },
  ): BuilderElement {
    const elementNumber = this.nextElementNumber
    this.nextElementNumber += 1
    const name = kind === 'img' ? `Image ${elementNumber}` : `Div ${elementNumber}`

    return {
      id: `${kind}-${elementNumber}`,
      name,
      kind,
      parentId: position.parentId,
      x: position.x,
      y: position.y,
      width: position.width ?? defaultElementSize.width,
      height: position.height ?? defaultElementSize.height,
      color: elementColors[(elementNumber - 1) % elementColors.length],
      visible: true,
      src: position.src,
      alt: position.alt,
    }
  }

  private cloneElementTree(
    element: BuilderElement,
    clonedParentId: string | null,
    cloneMap: Map<string, string>,
    clonedElements: Array<BuilderElement>,
  ) {
    const clone = this.createElement(element.kind, {
      parentId: clonedParentId ?? element.parentId,
      x: clonedParentId ? element.x : element.x + 24,
      y: clonedParentId ? element.y : element.y + 24,
      width: element.width,
      height: element.height,
      src: element.src,
      alt: element.alt,
    })

    clone.color = element.color
    clone.visible = element.visible
    cloneMap.set(element.id, clone.id)
    clonedElements.push(clone)

    getChildElements(element.id, this.state.elements).forEach((child) => {
      this.cloneElementTree(child, clone.id, cloneMap, clonedElements)
    })
  }

  private getSelectedElement() {
    return this.state.elements.find(
      (element) => element.id === this.state.selectedElementId,
    )
  }

  private commit(
    action: string,
    reducer: (state: ResearchWidgetState) => ResearchWidgetState,
  ) {
    const nextState = reducer(this.state)
    this.state = {
      ...nextState,
      history: [
        ...nextState.history,
        {
          id: this.nextHistoryId,
          action,
          at: new Intl.DateTimeFormat(undefined, {
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit',
          }).format(new Date()),
          snapshot: {
            elements: nextState.elements,
            selectedElementIds: nextState.selectedElementIds,
            grid: nextState.grid,
          },
        },
      ],
    }
    this.nextHistoryId += 1
    saveState(this.state)
    this.emit()
  }

  private emit() {
    this.listeners.forEach((listener) => listener())
    researchWidgetEventClient.emit('widget-state', this.state)
  }
}

export function getGeneratedCode(elements: Array<BuilderElement>) {
  const rootElements = getChildElements(null, elements)

  if (elements.length === 0) {
    return {
      html: '<!-- Drag a div from the widget to generate HTML. -->',
      css: '/* Drag a div from the widget to generate CSS. */',
    }
  }

  return {
    html: rootElements.map((element) => getElementHtml(element, elements)).join('\n'),
    css: elements
      .map((element) =>
        element.kind === 'img'
          ? [
          `.${getClassName(element)} {`,
          '  position: absolute;',
          `  left: ${element.x}px;`,
          `  top: ${element.y}px;`,
          `  width: ${element.width}px;`,
          `  height: ${element.height}px;`,
          '  object-fit: cover;',
          ...(element.visible ? [] : ['  display: none;']),
          '}',
        ].join('\n')
          : [
          `.${getClassName(element)} {`,
          '  position: absolute;',
          `  left: ${element.x}px;`,
          `  top: ${element.y}px;`,
          `  width: ${element.width}px;`,
          `  height: ${element.height}px;`,
          `  background: ${element.color};`,
          ...(element.visible ? [] : ['  display: none;']),
          '}',
        ].join('\n'),
      )
      .join('\n\n'),
  }
}

export function getChildElements(
  parentId: string | null,
  elements: Array<BuilderElement>,
) {
  return elements.filter((element) => element.parentId === parentId)
}

function getElementHtml(
  element: BuilderElement,
  elements: Array<BuilderElement>,
  depth = 0,
): string {
  const indent = '  '.repeat(depth)
  const children = getChildElements(element.id, elements)

  if (element.kind === 'img') {
    return `${indent}<img class="${getClassName(element)}" src="${element.src ?? ''}" alt="${element.alt ?? ''}" />`
  }

  if (children.length === 0) {
    return `${indent}<div class="${getClassName(element)}"></div>`
  }

  return [
    `${indent}<div class="${getClassName(element)}">`,
    children.map((child) => getElementHtml(child, elements, depth + 1)).join('\n'),
    `${indent}</div>`,
  ].join('\n')
}

function getClassName(element: BuilderElement) {
  return element.kind === 'img' ? `builder-image-${element.id}` : `builder-${element.id}`
}

function getAbsolutePosition(
  element: BuilderElement,
  elements: Array<BuilderElement>,
) {
  let x = element.x
  let y = element.y
  let currentElement = element

  while (currentElement.parentId) {
    const parent = elements.find((candidate) => candidate.id === currentElement.parentId)
    if (!parent) break

    x += parent.x
    y += parent.y
    currentElement = parent
  }

  return { x, y }
}

function isAncestor(
  possibleAncestorId: string,
  elementId: string,
  elements: Array<BuilderElement>,
) {
  let current = elements.find((element) => element.id === elementId)

  while (current?.parentId) {
    if (current.parentId === possibleAncestorId) return true
    current = elements.find((element) => element.id === current?.parentId)
  }

  return false
}

function hasSelectedAncestor(
  elementId: string,
  selectedIdSet: Set<string>,
  elements: Array<BuilderElement>,
) {
  let current = elements.find((element) => element.id === elementId)

  while (current?.parentId) {
    if (selectedIdSet.has(current.parentId)) return true
    current = elements.find((element) => element.id === current?.parentId)
  }

  return false
}

export const researchWidgetStore = new ResearchWidgetStore()

function loadSavedState(): ResearchWidgetState {
  if (typeof window === 'undefined') return createInitialState()

  try {
    const rawValue = window.localStorage.getItem(storageKey)
    if (!rawValue) return createInitialState()

    const parsedValue = JSON.parse(rawValue) as Partial<ResearchWidgetState>
    if (!Array.isArray(parsedValue.elements)) return createInitialState()

    const elements = parsedValue.elements.map((element) => ({
      ...element,
      kind: element.kind ?? 'div',
      parentId: element.parentId ?? null,
      visible: element.visible ?? true,
    }))
    const selectedElementIds =
      parsedValue.selectedElementIds ??
      (parsedValue.selectedElementId ? [parsedValue.selectedElementId] : [])

    return {
      ...createInitialState(),
      elements,
      selectedElementId: parsedValue.selectedElementId ?? selectedElementIds[0] ?? null,
      selectedElementIds,
      grid: {
        enabled: parsedValue.grid?.enabled ?? true,
        size: parsedValue.grid?.size ?? 16,
      },
    }
  } catch {
    return createInitialState()
  }
}

function saveState(state: ResearchWidgetState) {
  if (typeof window === 'undefined') return

  window.localStorage.setItem(
    storageKey,
    JSON.stringify({
      elements: state.elements,
      selectedElementId: state.selectedElementId,
      selectedElementIds: state.selectedElementIds,
      grid: state.grid,
    }),
  )
}

function clearSavedState() {
  if (typeof window === 'undefined') return

  window.localStorage.removeItem(storageKey)
}

function getNextElementNumber(elements: Array<BuilderElement>) {
  const highestElementNumber = elements.reduce((highest, element) => {
    const match = element.id.match(/^(?:div|img)-(\d+)$/)
    if (!match) return highest

    return Math.max(highest, Number(match[1]))
  }, 0)

  return highestElementNumber + 1
}

function reorderElementsWithinParents(
  elements: Array<BuilderElement>,
  selectedIdSet: Set<string>,
  direction: 'up' | 'down',
) {
  const nextElements = [...elements]
  const parentIds = Array.from(
    new Set(elements.map((element) => element.parentId ?? '__root__')),
  )

  parentIds.forEach((parentKey) => {
    const parentId = parentKey === '__root__' ? null : parentKey
    const siblingIds = nextElements
      .filter((element) => element.parentId === parentId)
      .map((element) => element.id)

    const orderedSiblingIds =
      direction === 'up'
        ? moveSelectedForward(siblingIds, selectedIdSet)
        : moveSelectedBackward(siblingIds, selectedIdSet)

    orderedSiblingIds.forEach((elementId, siblingIndex) => {
      const currentIndex = nextElements.findIndex(
        (element) => element.id === elementId,
      )
      const targetIndex = nextElements.findIndex((element) => {
        if (element.parentId !== parentId) return false
        const currentSiblingIndex = orderedSiblingIds.indexOf(element.id)
        return currentSiblingIndex >= siblingIndex
      })

      if (currentIndex === -1 || targetIndex === -1 || currentIndex === targetIndex) {
        return
      }

      const [element] = nextElements.splice(currentIndex, 1)
      const adjustedTargetIndex =
        currentIndex < targetIndex ? targetIndex - 1 : targetIndex
      nextElements.splice(adjustedTargetIndex, 0, element)
    })
  })

  return nextElements
}

function moveSelectedForward(ids: Array<string>, selectedIdSet: Set<string>) {
  const nextIds = [...ids]

  for (let index = nextIds.length - 2; index >= 0; index -= 1) {
    if (!selectedIdSet.has(nextIds[index]) || selectedIdSet.has(nextIds[index + 1])) {
      continue
    }

    ;[nextIds[index], nextIds[index + 1]] = [nextIds[index + 1], nextIds[index]]
  }

  return nextIds
}

function moveSelectedBackward(ids: Array<string>, selectedIdSet: Set<string>) {
  const nextIds = [...ids]

  for (let index = 1; index < nextIds.length; index += 1) {
    if (!selectedIdSet.has(nextIds[index]) || selectedIdSet.has(nextIds[index - 1])) {
      continue
    }

    ;[nextIds[index - 1], nextIds[index]] = [nextIds[index], nextIds[index - 1]]
  }

  return nextIds
}
