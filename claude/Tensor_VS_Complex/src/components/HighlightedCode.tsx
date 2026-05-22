import { forwardRef } from "react";

type SyntaxType =
  | "builtin"
  | "comment"
  | "function"
  | "keyword"
  | "number"
  | "operator"
  | "plain"
  | "string";

type CodePart = {
  text: string;
  type: SyntaxType;
};

type HighlightedCodeProps = {
  code: string;
};

const KEYWORDS = new Set<string>([
  "as",
  "break",
  "case",
  "const",
  "continue",
  "crate",
  "default",
  "else",
  "extern",
  "export",
  "fn",
  "for",
  "from",
  "function",
  "if",
  "import",
  "let",
  "mut",
  "new",
  "of",
  "pub",
  "return",
  "static",
  "struct",
  "switch",
  "type",
  "unsafe",
  "var",
  "while",
]);

const BUILT_INS = new Set<string>([
  "Array",
  "Float32Array",
  "Int32Array",
  "Math",
  "Number",
  "Object",
  "WebAssembly",
  "f32",
  "usize",
]);

const TOKEN_PATTERN =
  /\/\/[^\n]*|\/\*[\s\S]*?\*\/|"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'|`(?:\\[\s\S]|[^`\\])*`|\b[A-Za-z_$][\w$]*\b|\b\d+(?:\.\d+)?\b|[{}()[\].,;:+\-*/%=<>!&|?]+/g;

function classifyToken(token: string, line: string, offset: number): SyntaxType {
  if (token.startsWith("//") || token.startsWith("/*")) return "comment";
  if (/^["'`]/.test(token)) return "string";
  if (/^\d/.test(token)) return "number";
  if (KEYWORDS.has(token)) return "keyword";
  if (BUILT_INS.has(token)) return "builtin";
  if (line.slice(offset + token.length).trimStart().startsWith("(")) return "function";
  if (/^[{}()[\].,;:+\-*/%=<>!&|?]+$/.test(token)) return "operator";
  return "plain";
}

function highlightLine(line: string): CodePart[] {
  const parts: CodePart[] = [];
  let lastIndex = 0;

  for (const match of line.matchAll(TOKEN_PATTERN)) {
    const token = match[0];
    const offset = match.index ?? 0;

    if (offset > lastIndex) {
      parts.push({ text: line.slice(lastIndex, offset), type: "plain" });
    }

    parts.push({ text: token, type: classifyToken(token, line, offset) });
    lastIndex = offset + token.length;
  }

  if (lastIndex < line.length) {
    parts.push({ text: line.slice(lastIndex), type: "plain" });
  }

  return parts;
}

const HighlightedCode = forwardRef<HTMLPreElement, HighlightedCodeProps>(
  function HighlightedCode({ code }, ref) {
    const lines = code.replace(/\n$/, "").split("\n");

    return (
      <pre className="highlighted-code" ref={ref}>
        <code>
          {lines.map((line, lineIndex) => (
            <span className="code-line" key={`${lineIndex}-${line}`}>
              <span className="line-number">{lineIndex + 1}</span>
              <span className="line-code">
                {highlightLine(line).map((part, partIndex) => (
                  <span className={`syntax-${part.type}`} key={`${partIndex}-${part.text}`}>
                    {part.text}
                  </span>
                ))}
              </span>
            </span>
          ))}
        </code>
      </pre>
    );
  },
);

export default HighlightedCode;
