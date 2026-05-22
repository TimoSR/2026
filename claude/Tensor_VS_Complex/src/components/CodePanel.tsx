import { useEffect, useLayoutEffect, useRef, useState } from "react";
import type { CSSProperties } from "react";
import { CODE_KEYS, CODE_NAMES, CODES } from "../data/codeSamples";
import type { CodeSampleKey } from "../data/codeSamples";
import HighlightedCode from "./HighlightedCode";

const COPY_RESET_MS = 1200;

type CodeViewStyle = CSSProperties & {
  "--code-scrollbar-space": string;
};

type CodePanelProps = {
  codeMode: CodeSampleKey;
  onCodeModeChange: (codeMode: CodeSampleKey) => void;
};

function copyWithTextarea(text: string): void {
  const textarea = document.createElement("textarea");
  textarea.value = text;
  textarea.setAttribute("readonly", "");
  textarea.style.position = "fixed";
  textarea.style.top = "0";
  textarea.style.left = "0";
  textarea.style.width = "1px";
  textarea.style.height = "1px";
  textarea.style.opacity = "0";

  const selection = document.getSelection();
  const previousRange =
    selection && selection.rangeCount > 0 ? selection.getRangeAt(0) : null;

  document.body.appendChild(textarea);
  textarea.focus();
  textarea.select();
  textarea.setSelectionRange(0, textarea.value.length);

  const copied = document.execCommand("copy");
  textarea.remove();

  if (previousRange && selection) {
    selection.removeAllRanges();
    selection.addRange(previousRange);
  }

  if (!copied) {
    throw new Error("Copy command failed");
  }
}

async function copyText(text: string): Promise<void> {
  try {
    if (navigator.clipboard?.writeText) {
      await navigator.clipboard.writeText(text);
      return;
    }
  } catch {
    // Fall back below when browser permissions block navigator.clipboard.
  }

  copyWithTextarea(text);
}

export default function CodePanel({ codeMode, onCodeModeChange }: CodePanelProps) {
  const [copied, setCopied] = useState(false);
  const [scrollbarOffset, setScrollbarOffset] = useState(0);
  const codeRef = useRef<HTMLPreElement | null>(null);
  const resetTimeoutRef = useRef<number | null>(null);

  useEffect(() => {
    setCopied(false);
  }, [codeMode]);

  useEffect(() => {
    return () => {
      if (resetTimeoutRef.current !== null) {
        window.clearTimeout(resetTimeoutRef.current);
      }
    };
  }, []);

  useLayoutEffect(() => {
    const codeElement = codeRef.current;
    if (!codeElement) return;

    const updateScrollbarOffset = () => {
      const hasVerticalScrollbar = codeElement.scrollHeight > codeElement.clientHeight + 1;
      const nextOffset = hasVerticalScrollbar
        ? Math.max(0, codeElement.offsetWidth - codeElement.clientWidth)
        : 0;

      setScrollbarOffset((currentOffset) =>
        currentOffset === nextOffset ? currentOffset : nextOffset,
      );
    };

    updateScrollbarOffset();

    const resizeObserver = new ResizeObserver(updateScrollbarOffset);
    resizeObserver.observe(codeElement);
    resizeObserver.observe(codeElement.querySelector("code") ?? codeElement);
    window.addEventListener("resize", updateScrollbarOffset);

    return () => {
      resizeObserver.disconnect();
      window.removeEventListener("resize", updateScrollbarOffset);
    };
  }, [codeMode]);

  const copyCurrentCode = async () => {
    try {
      await copyText(CODES[codeMode]);
      setCopied(true);
    } catch {
      setCopied(false);
      return;
    }

    if (resetTimeoutRef.current !== null) {
      window.clearTimeout(resetTimeoutRef.current);
    }

    resetTimeoutRef.current = window.setTimeout(() => {
      setCopied(false);
      resetTimeoutRef.current = null;
    }, COPY_RESET_MS);
  };

  const codeViewStyle: CodeViewStyle = {
    "--code-scrollbar-space": `${scrollbarOffset}px`,
  };

  return (
    <section className="panel code-panel">
      <div className="code-panel-toolbar">
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
      </div>
      <div className="code-view" style={codeViewStyle}>
        <button
          type="button"
          className={`copy-code-button${copied ? " is-copied" : ""}`}
          aria-label={copied ? "Code copied" : `Copy ${CODE_NAMES[codeMode]} code`}
          title={copied ? "Copied" : "Copy code"}
          onClick={copyCurrentCode}
        >
          {copied ? (
            <svg
              aria-hidden="true"
              fill="none"
              height="16"
              viewBox="0 0 24 24"
              width="16"
            >
              <path d="m5 12 4 4L19 6" />
            </svg>
          ) : (
            <svg
              aria-hidden="true"
              fill="none"
              height="16"
              viewBox="0 0 24 24"
              width="16"
            >
              <rect x="9" y="9" width="10" height="10" rx="2" />
              <path d="M5 15V7a2 2 0 0 1 2-2h8" />
            </svg>
          )}
        </button>
        <HighlightedCode code={CODES[codeMode]} ref={codeRef} />
      </div>
    </section>
  );
}
