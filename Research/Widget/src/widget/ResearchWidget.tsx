import {
  ChevronDown,
  ChevronUp,
  Code2,
  Copy,
  Eye,
  EyeOff,
  Grid3X3,
  Layers,
  MessageSquare,
  Move,
  Plus,
  RotateCcw,
  Save,
  Square,
  Unlink,
  X,
} from 'lucide-react'
import {
  type CSSProperties,
  type PointerEvent as ReactPointerEvent,
  useEffect,
  useMemo,
  useRef,
  useState,
  useSyncExternalStore,
} from 'react'

import {
  getChildElements,
  getGeneratedCode,
  researchWidgetStore,
  type BuilderElement,
} from './researchWidgetStore'

type PaletteDrag = {
  offsetX: number
  offsetY: number
}

const defaultDragSize = {
  width: 160,
  height: 112,
}

export function ComponentBuilderWidget() {
  const [isOpen, setIsOpen] = useState(false)
  const [dragPreview, setDragPreview] = useState<{ x: number; y: number } | null>(
    null,
  )
  const paletteDrag = useRef<PaletteDrag | null>(null)
  const state = useSyncExternalStore(
    researchWidgetStore.subscribe,
    researchWidgetStore.getSnapshot,
    researchWidgetStore.getSnapshot,
  )
  const selectedElement = state.elements.find(
    (element) => element.id === state.selectedElementId,
  )
  const code = useMemo(() => getGeneratedCode(state.elements), [state.elements])

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 's') {
        event.preventDefault()
        researchWidgetStore.save()
      }
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'd') {
        event.preventDefault()
        researchWidgetStore.duplicateSelectedElements()
      }
    }

    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [])

  const startPaletteDrag = (event: ReactPointerEvent<HTMLButtonElement>) => {
    event.preventDefault()
    event.currentTarget.setPointerCapture(event.pointerId)
    paletteDrag.current = {
      offsetX: defaultDragSize.width / 2,
      offsetY: defaultDragSize.height / 2,
    }
    setDragPreview({
      x: event.clientX - defaultDragSize.width / 2,
      y: event.clientY - defaultDragSize.height / 2,
    })
  }

  const movePaletteDrag = (event: ReactPointerEvent<HTMLButtonElement>) => {
    if (!paletteDrag.current) return

    setDragPreview({
      x: event.clientX - paletteDrag.current.offsetX,
      y: event.clientY - paletteDrag.current.offsetY,
    })
  }

  const stopPaletteDrag = (event: ReactPointerEvent<HTMLButtonElement>) => {
    const drag = paletteDrag.current
    if (!drag) return

    event.currentTarget.releasePointerCapture(event.pointerId)
    paletteDrag.current = null
    setDragPreview(null)

    const canvas = document.querySelector<HTMLElement>('[data-builder-canvas]')
    if (!canvas) return

    const rect = canvas.getBoundingClientRect()
    const isInside =
      event.clientX >= rect.left &&
      event.clientX <= rect.right &&
      event.clientY >= rect.top &&
      event.clientY <= rect.bottom

    if (!isInside) return

    const x = event.clientX - rect.left - drag.offsetX
    const y = event.clientY - rect.top - drag.offsetY
    const placed = snapPlacement(
      {
        x: clamp(x, 0, rect.width - defaultDragSize.width),
        y: clamp(y, 0, rect.height - defaultDragSize.height),
      },
      state.grid.enabled ? state.grid.size : null,
    )
    researchWidgetStore.placeDiv(placed.x, placed.y)
  }

  const launcherLabel = isOpen
    ? 'Close component builder'
    : 'Open component builder'

  return (
    <aside className="widget-shell" aria-label="Component builder widget">
      {dragPreview ? (
        <div
          className="drag-preview"
          style={{
            left: dragPreview.x,
            top: dragPreview.y,
            width: defaultDragSize.width,
            height: defaultDragSize.height,
            background: selectedElement?.color ?? '#4f8cff',
          }}
        >
          div
        </div>
      ) : null}

      {isOpen ? (
        <section className="widget-panel">
          <header className="widget-panel-header">
            <div>
              <p className="eyebrow">HTML component builder</p>
              <h2>Palette</h2>
            </div>
            <button
              type="button"
              className="icon-button"
              aria-label="Close widget"
              onClick={() => setIsOpen(false)}
            >
              <X size={18} aria-hidden />
            </button>
          </header>

          <div className="palette-area" aria-label="Component palette">
            <button
              type="button"
              className="palette-tile"
              onPointerDown={startPaletteDrag}
              onPointerMove={movePaletteDrag}
              onPointerUp={stopPaletteDrag}
              onPointerCancel={stopPaletteDrag}
            >
              <Square size={18} aria-hidden />
              <span>Div</span>
              <small>Drag onto the website canvas</small>
              <Move size={16} aria-hidden />
            </button>
          </div>

          <div className="builder-toolbar" aria-label="Builder controls">
            <button type="button" onClick={() => researchWidgetStore.addDiv()}>
              <Plus size={16} aria-hidden />
              Add div
            </button>
            <button
              type="button"
              aria-pressed={state.grid.enabled}
              onClick={() => researchWidgetStore.toggleGrid()}
            >
              <Grid3X3 size={16} aria-hidden />
              Grid
            </button>
            <label className="color-control">
              <span>Color</span>
              <input
                type="color"
                value={selectedElement?.color ?? '#4f8cff'}
                disabled={!selectedElement}
                onChange={(event) =>
                  researchWidgetStore.updateSelectedElement({
                    color: event.currentTarget.value,
                  })
                }
              />
            </label>
            <button type="button" onClick={() => researchWidgetStore.reset()}>
              <RotateCcw size={16} aria-hidden />
              Reset
            </button>
            <button type="button" onClick={() => researchWidgetStore.save()}>
              <Save size={16} aria-hidden />
              Save
            </button>
            <button
              type="button"
              disabled={state.selectedElementIds.length === 0}
              onClick={() => researchWidgetStore.duplicateSelectedElements()}
            >
              <Copy size={16} aria-hidden />
              Duplicate
            </button>
          </div>

          <div className="layer-toolbar" aria-label="Layer controls">
            <button
              type="button"
              disabled={state.selectedElementIds.length < 2}
              onClick={() => researchWidgetStore.nestSelectedElements()}
            >
              <Layers size={16} aria-hidden />
              Nest selected
            </button>
            <button
              type="button"
              disabled={state.selectedElementIds.length === 0}
              onClick={() => researchWidgetStore.unnestSelectedElements()}
            >
              <Unlink size={16} aria-hidden />
              Unnest
            </button>
            <button
              type="button"
              disabled={state.selectedElementIds.length === 0}
              onClick={() => researchWidgetStore.moveSelectedLayers('up')}
            >
              <ChevronUp size={16} aria-hidden />
              Up
            </button>
            <button
              type="button"
              disabled={state.selectedElementIds.length === 0}
              onClick={() => researchWidgetStore.moveSelectedLayers('down')}
            >
              <ChevronDown size={16} aria-hidden />
              Down
            </button>
            <button
              type="button"
              disabled={state.selectedElementIds.length === 0}
              onClick={() => researchWidgetStore.toggleSelectedVisibility()}
            >
              {selectedElement?.visible === false ? (
                <Eye size={16} aria-hidden />
              ) : (
                <EyeOff size={16} aria-hidden />
              )}
              {selectedElement?.visible === false ? 'Show' : 'Hide'}
            </button>
          </div>

          <p
            key={state.saveCount}
            className={state.lastSavedAt ? 'save-status saved' : 'save-status'}
            aria-live="polite"
          >
            {state.lastSavedAt
              ? `Saved at ${state.lastSavedAt}`
              : 'Not saved yet'}
          </p>

          <section className="element-list" aria-label="Placed elements">
            <h3>Elements</h3>
            {state.elements.length > 0 ? (
              <div>
                {getChildElements(null, state.elements).map((element) => (
                  <LayerListItem
                    key={element.id}
                    element={element}
                    elements={state.elements}
                    selectedElementIds={state.selectedElementIds}
                    selectedElementId={state.selectedElementId}
                    depth={0}
                  />
                ))}
              </div>
            ) : (
              <p>Drag a div from the palette onto the website canvas.</p>
            )}
          </section>

          <section className="code-output" aria-label="Generated code">
            <div>
              <h3>
                <Code2 size={16} aria-hidden />
                HTML
              </h3>
              <pre>{code.html}</pre>
            </div>
            <div>
              <h3>
                <Code2 size={16} aria-hidden />
                CSS
              </h3>
              <pre>{code.css}</pre>
            </div>
          </section>
        </section>
      ) : null}

      <button
        type="button"
        className="widget-launcher"
        aria-expanded={isOpen}
        aria-label={launcherLabel}
        onClick={() => setIsOpen((current) => !current)}
      >
        {isOpen ? (
          <X size={20} aria-hidden />
        ) : (
          <MessageSquare size={20} aria-hidden />
        )}
        <span>Builder</span>
      </button>
    </aside>
  )
}

function LayerListItem({
  element,
  elements,
  selectedElementIds,
  selectedElementId,
  depth,
}: {
  element: BuilderElement
  elements: Array<BuilderElement>
  selectedElementIds: Array<string>
  selectedElementId: string | null
  depth: number
}) {
  const children = getChildElements(element.id, elements)
  const isSelected = selectedElementIds.includes(element.id)
  const isPrimarySelected = element.id === selectedElementId

  return (
    <>
      <button
        type="button"
        className={[
          isSelected ? 'selected' : '',
          element.visible ? '' : 'hidden-layer',
        ]
          .filter(Boolean)
          .join(' ')}
        style={{ '--layer-depth': depth } as CSSProperties}
        onClick={(event) =>
          researchWidgetStore.selectElement(
            element.id,
            event.ctrlKey || event.metaKey || event.shiftKey,
          )
        }
      >
        <span>
          <strong>
            {element.name}
            {isPrimarySelected ? ' *' : ''}
          </strong>
          <small>
            {element.parentId ? 'Child' : 'Root'} | {element.width} x{' '}
            {element.height} at {element.x}, {element.y}
          </small>
        </span>
        <i style={{ background: element.color }} />
      </button>
      {children.map((child) => (
        <LayerListItem
          key={child.id}
          element={child}
          elements={elements}
          selectedElementIds={selectedElementIds}
          selectedElementId={selectedElementId}
          depth={depth + 1}
        />
      ))}
    </>
  )
}

function snapPlacement(
  point: { x: number; y: number },
  gridSize: number | null,
) {
  if (!gridSize) return point

  return {
    x: Math.round(point.x / gridSize) * gridSize,
    y: Math.round(point.y / gridSize) * gridSize,
  }
}

function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max)
}
