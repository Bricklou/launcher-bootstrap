import React, { Component } from "react";
import style from "./button.module.css";
import type { LucideIcon } from "lucide-react";

export interface ButtonProps {
  children?: React.ReactNode;
  onClick?: () => void;
  icon?: React.ReactElement<LucideIcon>;
  type?: React.ButtonHTMLAttributes<HTMLButtonElement>["type"];
}

export function Button({
  type = "button",
  children,
  icon,
  ...props
}: ButtonProps): JSX.Element {
  return (
    <button className={style.button} type={type}>
      {icon}
      {children}
    </button>
  );
}
