import React from "react";
import { createPortal } from "react-dom";
import { Button } from "../button/Button";
import classNames from "classnames";

interface ModalProps {
  title?: string;
  children?: React.ReactNode;
  show?: boolean;
  onClose?: () => void;
  actions?: Array<{
    label: string;
    onClick?: () => void;
    btnTypeClass?: string;
  }>;
}

export function Modal(props: ModalProps): JSX.Element {
  const onClickWrapper = (onClick: () => void) => {
    return () => {
      props.onClose?.();
      onClick();
    };
  };
  return createPortal(
    <div>
      <div
        className={classNames("modal cursor-pointer", {
          "modal-open": props.show,
        })}
      >
        <label className="modal-box relative" htmlFor="">
          <h3 className="text-lg font-bold">{props.title}</h3>
          {props.children}

          <div className="modal-action">
            {props.actions?.map((action) => (
              <Button
                onClick={
                  action.onClick ? onClickWrapper(action.onClick) : undefined
                }
                btnTypeClass={action.btnTypeClass}
              >
                {action.label}
              </Button>
            ))}
          </div>
        </label>
      </div>
    </div>,
    document.body
  );
}
