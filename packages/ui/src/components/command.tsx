import { ReactNode } from "react";

export const Command = (props: {
    children: ReactNode
}) => {
  return (
    <pre className="p-2 border rounded">
      {props.children}
    </pre>
  );
};
