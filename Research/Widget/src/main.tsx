import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'

import App from './App'
import './styles.css'
import { ComponentBuilderDevtools } from './widget'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
    <ComponentBuilderDevtools />
  </StrictMode>,
)
