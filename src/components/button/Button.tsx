import React, { Component } from "react";
import style from "./button.module.css";
import type { LucideIcon } from "lucide-react";

export type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {
  icon?: React.ReactElement<LucideIcon>;
};

export function Button({
  type = "button",
  children,
  icon,
  ...props
}: ButtonProps): JSX.Element {
  return (
    <button className={style.button} type={type} {...props}>
      {icon}
      {children}
    </button>
  );
}
