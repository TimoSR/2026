import { EventClient } from '@tanstack/devtools-event-client'

import type { ResearchWidgetState } from './researchWidgetStore'

type ResearchWidgetEventMap = {
  'widget-state': ResearchWidgetState
}

class ResearchWidgetEventClient extends EventClient<ResearchWidgetEventMap> {
  constructor() {
    super({
      pluginId: 'research-widget',
    })
  }
}

export const researchWidgetEventClient = new ResearchWidgetEventClient()
