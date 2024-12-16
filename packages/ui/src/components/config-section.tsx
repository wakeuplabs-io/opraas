import React from "react";

export const ConfigSection: React.FC<{
  id: string;
  title: string;
  description?: string;
  children: React.ReactNode;
}> = (props) => {
  return (
    <section className="space-y-6 py-6">
      <div>
        <h3 id={props.id} className="text-lg font-semibold">
          {props.title}
        </h3>
        {props.description && (
          <p className="text-sm mt-2">{props.description}</p>
        )}
      </div>

      <div className="space-y-4">{props.children}</div>
    </section>
  );
};
