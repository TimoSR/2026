import { CODE_KEYS, CODE_NAMES, CODES } from "../data/codeSamples";
import type { CodeSampleKey } from "../data/codeSamples";
import HighlightedCode from "./HighlightedCode";

type CodePanelProps = {
  codeMode: CodeSampleKey;
  onCodeModeChange: (codeMode: CodeSampleKey) => void;
};

export default function CodePanel({ codeMode, onCodeModeChange }: CodePanelProps) {
  return (
    <section className="panel code-panel">
      <div className="code-tabs" role="tablist" aria-label="Implementation code">
        {CODE_KEYS.map((key) => (
          <button
            key={key}
            type="button"
            role="tab"
            aria-selected={codeMode === key}
            className={codeMode === key ? "is-active" : ""}
            onClick={() => onCodeModeChange(key)}
          >
            {CODE_NAMES[key]}
          </button>
        ))}
      </div>
      <HighlightedCode code={CODES[codeMode]} />
    </section>
  );
}
