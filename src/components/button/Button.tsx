import React, { Component } from "react";
import style from "./button.module.css";
import type { LucideIcon } from "lucide-react";
import classNames from "classnames";

export type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {
  icon?: React.ReactElement<LucideIcon>;
  btnTypeClass?: string;
};

export function Button({
  type = "button",
  children,
  icon,
  className,
  btnTypeClass = "btn-primary",
  ...props
}: ButtonProps): JSX.Element {
  return (
    <button
      className={classNames(style.button, btnTypeClass, className)}
      type={type}
      {...props}
    >
      {icon}
      {children}
    </button>
  );
}
