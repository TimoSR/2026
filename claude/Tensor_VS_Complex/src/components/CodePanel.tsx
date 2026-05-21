import { useState } from "react";
import { CODES } from "../data/codeSamples";
import type { AlgorithmKey } from "../types";
import { KEYS, NAMES } from "../wave-algorithms";
import HighlightedCode from "./HighlightedCode";

export default function CodePanel() {
  const [codeMode, setCodeMode] = useState<AlgorithmKey>("cx");

  return (
    <section className="panel code-panel">
      <div className="code-tabs" role="tablist" aria-label="Implementation code">
        {KEYS.map((key) => (
          <button
            key={key}
            type="button"
            role="tab"
            aria-selected={codeMode === key}
            className={codeMode === key ? "is-active" : ""}
            onClick={() => setCodeMode(key)}
          >
            {NAMES[key]}
          </button>
        ))}
      </div>
      <HighlightedCode code={CODES[codeMode]} />
    </section>
  );
}
