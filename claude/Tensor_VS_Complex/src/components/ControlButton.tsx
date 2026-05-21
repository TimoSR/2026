import type { ButtonHTMLAttributes } from "react";

type ControlButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  active?: boolean;
};

export default function ControlButton({
  active = false,
  children,
  className = "",
  ...buttonProps
}: ControlButtonProps) {
  const classes = ["control-button", active ? "is-active" : "", className]
    .filter(Boolean)
    .join(" ");

  return (
    <button type="button" className={classes} {...buttonProps}>
      {children}
    </button>
  );
}
