import { TanStackDevtools } from '@tanstack/react-devtools'
import { useEffect, useMemo, useState } from 'react'

import { researchWidgetEventClient } from './researchWidgetEventClient'
import {
  getGeneratedCode,
  researchWidgetStore,
  type ResearchWidgetState,
} from './researchWidgetStore'

export const componentBuilderPlugin = {
  id: 'component-builder-widget',
  name: 'Component Builder',
  render: <ComponentBuilderDevtoolsPanel />,
}

export function ComponentBuilderDevtools() {
  return <TanStackDevtools plugins={[componentBuilderPlugin]} />
}

export function ComponentBuilderDevtoolsPanel() {
  const [state, setState] = useState<ResearchWidgetState>(
    researchWidgetStore.getSnapshot(),
  )
  const selectedElement = state.elements.find(
    (element) => element.id === state.selectedElementId,
  )
  const code = useMemo(() => getGeneratedCode(state.elements), [state.elements])

  useEffect(() => {
    setState(researchWidgetStore.getSnapshot())
    return researchWidgetEventClient.on('widget-state', (event) => {
      setState(event.payload)
    })
  }, [])

  return (
    <div className="devtools-panel">
      <header>
        <div>
          <p>Component Builder</p>
          <h2>
            {selectedElement
              ? `${selectedElement.name} selected`
              : `${state.elements.length} elements`}
          </h2>
        </div>
        <span>{state.grid.enabled ? 'grid on' : 'grid off'}</span>
      </header>

      <div className="devtools-selected">
        <span>Geometry</span>
        <strong>
          {selectedElement
            ? `${selectedElement.width} x ${selectedElement.height} at ${selectedElement.x}, ${selectedElement.y}`
            : 'No div selected'}
        </strong>
      </div>

      <div className="devtools-code">
        <section>
          <h3>HTML</h3>
          <pre>{code.html}</pre>
        </section>
        <section>
          <h3>CSS</h3>
          <pre>{code.css}</pre>
        </section>
      </div>

      <footer>
        <button type="button" onClick={() => researchWidgetStore.addDiv()}>
          Add div
        </button>
        <button type="button" onClick={() => researchWidgetStore.toggleGrid()}>
          Toggle grid
        </button>
      </footer>
    </div>
  )
}
