import { MousePointer2, Plus } from 'lucide-react'
import {
  type CSSProperties,
  type DragEvent as ReactDragEvent,
  type PointerEvent as ReactPointerEvent,
  useRef,
  useSyncExternalStore,
} from 'react'

import {
  getChildElements,
  researchWidgetStore,
  type BuilderElement,
  type BuilderGrid,
} from './researchWidgetStore'

type DragMode = 'move' | 'resize'

type DragState = {
  mode: DragMode
  startClientX: number
  startClientY: number
  initialElement: BuilderElement
  canvasWidth: number
  canvasHeight: number
}

type ElementLayerProps = {
  element: BuilderElement
  elements: Array<BuilderElement>
  selectedElementIds: Array<string>
  selectedElementId: string | null
  grid: BuilderGrid
  onPointerMove: (event: ReactPointerEvent<HTMLDivElement>) => void
  onPointerUp: (event: ReactPointerEvent<HTMLDivElement>) => void
  onStartInteraction: (
    event: ReactPointerEvent<HTMLDivElement>,
    mode: DragMode,
    element: BuilderElement,
  ) => void
  onImageDrop: (
    event: ReactDragEvent<HTMLDivElement>,
    element: BuilderElement,
  ) => void
}

const minElementSize = 32

export function ComponentBuilderCanvas() {
  const dragState = useRef<DragState | null>(null)
  const state = useSyncExternalStore(
    researchWidgetStore.subscribe,
    researchWidgetStore.getSnapshot,
    researchWidgetStore.getSnapshot,
  )

  const startInteraction = (
    event: ReactPointerEvent<HTMLDivElement>,
    mode: DragMode,
    element: BuilderElement,
  ) => {
    const parent = element.parentId
      ? document.querySelector<HTMLElement>(`[data-element-id="${element.parentId}"]`)
      : event.currentTarget.closest<HTMLElement>('[data-builder-canvas]')
    if (!parent) return

    const rect = parent.getBoundingClientRect()
    event.preventDefault()
    event.stopPropagation()
    researchWidgetStore.selectElement(
      element.id,
      event.ctrlKey || event.metaKey || event.shiftKey,
    )
    event.currentTarget.setPointerCapture(event.pointerId)
    dragState.current = {
      mode,
      startClientX: event.clientX,
      startClientY: event.clientY,
      initialElement: element,
      canvasWidth: rect.width,
      canvasHeight: rect.height,
    }
  }

  const updateInteraction = (event: ReactPointerEvent<HTMLDivElement>) => {
    const drag = dragState.current
    if (!drag) return

    const deltaX = event.clientX - drag.startClientX
    const deltaY = event.clientY - drag.startClientY

    if (drag.mode === 'move') {
      researchWidgetStore.updateElement(
        drag.initialElement.id,
        snapElement(
          {
            ...drag.initialElement,
            x: clamp(
              drag.initialElement.x + deltaX,
              0,
              drag.canvasWidth - drag.initialElement.width,
            ),
            y: clamp(
              drag.initialElement.y + deltaY,
              0,
              drag.canvasHeight - drag.initialElement.height,
            ),
          },
          state.grid,
          drag.canvasWidth,
          drag.canvasHeight,
        ),
      )
      return
    }

    researchWidgetStore.updateElement(
      drag.initialElement.id,
      snapElement(
        {
          ...drag.initialElement,
          width: clamp(
            drag.initialElement.width + deltaX,
            minElementSize,
            drag.canvasWidth - drag.initialElement.x,
          ),
          height: clamp(
            drag.initialElement.height + deltaY,
            minElementSize,
            drag.canvasHeight - drag.initialElement.y,
          ),
        },
        state.grid,
        drag.canvasWidth,
        drag.canvasHeight,
      ),
    )
  }

  const stopInteraction = (event: ReactPointerEvent<HTMLDivElement>) => {
    if (dragState.current && event.currentTarget.hasPointerCapture(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId)
    }
    dragState.current = null
  }

  const handleImageDrop = (
    event: ReactDragEvent<HTMLDivElement>,
    element: BuilderElement,
  ) => {
    if (element.kind !== 'div') return

    const file = Array.from(event.dataTransfer.files).find((candidate) =>
      candidate.type.startsWith('image/'),
    )
    if (!file) return

    event.preventDefault()
    event.stopPropagation()

    const rect = event.currentTarget.getBoundingClientRect()
    const x = event.clientX - rect.left
    const y = event.clientY - rect.top
    const reader = new FileReader()

    reader.addEventListener('load', () => {
      if (typeof reader.result !== 'string') return

      researchWidgetStore.addImageToParent(element.id, {
        src: reader.result,
        alt: file.name,
        x: state.grid.enabled ? snap(x, state.grid.size) : x,
        y: state.grid.enabled ? snap(y, state.grid.size) : y,
      })
    })
    reader.readAsDataURL(file)
  }

  return (
    <section
      className={
        state.grid.enabled
          ? 'blank-site builder-drop-canvas grid-on'
          : 'blank-site builder-drop-canvas'
      }
      aria-label="Blank website canvas"
      data-builder-canvas
      style={
        {
          '--grid-size': `${state.grid.size}px`,
        } as CSSProperties
      }
    >
      <div className="canvas-copy">
        <p className="eyebrow">Blank website</p>
        <h1>Drag a div from the widget onto this page</h1>
        <p>
          The placed element is edited directly here. Move and resize it on the
          page, and the widget updates the generated HTML/CSS.
        </p>
      </div>

      {getChildElements(null, state.elements).map((element) => (
        <ElementLayer
          key={element.id}
          element={element}
          elements={state.elements}
          selectedElementIds={state.selectedElementIds}
          selectedElementId={state.selectedElementId}
          grid={state.grid}
          onPointerMove={updateInteraction}
          onPointerUp={stopInteraction}
          onStartInteraction={startInteraction}
          onImageDrop={handleImageDrop}
        />
      ))}

      {state.elements.length === 0 ? (
        <button
          type="button"
          className="empty-canvas-action"
          onClick={() => researchWidgetStore.addDiv()}
        >
          <Plus size={16} aria-hidden />
          Place a div
        </button>
      ) : null}
    </section>
  )
}

function ElementLayer({
  element,
  elements,
  selectedElementIds,
  selectedElementId,
  grid,
  onPointerMove,
  onPointerUp,
  onStartInteraction,
  onImageDrop,
}: ElementLayerProps) {
  const isSelected = selectedElementIds.includes(element.id)
  const isPrimarySelected = element.id === selectedElementId
  const children = getChildElements(element.id, elements)

  return (
    <div
      className={[
        'placed-element',
        element.kind === 'img' ? 'image-element' : '',
        isSelected ? 'selected' : '',
      ]
        .filter(Boolean)
        .join(' ')}
      data-element-id={element.id}
      style={{
        left: element.x,
        top: element.y,
        width: element.width,
        height: element.height,
        background: element.kind === 'img' ? 'transparent' : element.color,
        display: element.visible ? undefined : 'none',
      }}
      onPointerDown={(event) => onStartInteraction(event, 'move', element)}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      onPointerCancel={onPointerUp}
      onDragOver={(event) => {
        if (element.kind === 'div') {
          event.preventDefault()
        }
      }}
      onDrop={(event) => onImageDrop(event, element)}
    >
      {element.kind === 'img' && element.src ? (
        <img src={element.src} alt={element.alt ?? element.name} />
      ) : null}
      <span>
        <MousePointer2 size={14} aria-hidden />
        {element.name}
      </span>
      {isPrimarySelected ? (
        <div
          className="resize-handle"
          aria-label={`Resize ${element.name}`}
          role="button"
          tabIndex={0}
          onPointerDown={(event) => {
            event.stopPropagation()
            onStartInteraction(event, 'resize', element)
          }}
        />
      ) : null}
      {children.map((child) => (
        <ElementLayer
          key={child.id}
          element={child}
          elements={elements}
          selectedElementIds={selectedElementIds}
          selectedElementId={selectedElementId}
          grid={grid}
          onPointerMove={onPointerMove}
          onPointerUp={onPointerUp}
          onStartInteraction={onStartInteraction}
          onImageDrop={onImageDrop}
        />
      ))}
    </div>
  )
}

function snapElement(
  element: BuilderElement,
  grid: BuilderGrid,
  canvasWidth: number,
  canvasHeight: number,
) {
  if (!grid.enabled) return element

  const snappedX = snap(element.x, grid.size)
  const snappedY = snap(element.y, grid.size)
  const snappedRight = snap(element.x + element.width, grid.size)
  const snappedBottom = snap(element.y + element.height, grid.size)

  return {
    ...element,
    x: clamp(snappedX, 0, canvasWidth - minElementSize),
    y: clamp(snappedY, 0, canvasHeight - minElementSize),
    width: clamp(snappedRight - snappedX, minElementSize, canvasWidth - snappedX),
    height: clamp(
      snappedBottom - snappedY,
      minElementSize,
      canvasHeight - snappedY,
    ),
  }
}

function snap(value: number, gridSize: number) {
  return Math.round(value / gridSize) * gridSize
}

function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max)
}
