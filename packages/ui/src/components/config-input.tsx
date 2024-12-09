import React, { forwardRef } from "react";

export const ConfigInput = forwardRef<
  HTMLInputElement,
  {
    title: string;
    description: string;
    defaultValue: string;
  } & React.InputHTMLAttributes<HTMLInputElement>
>((props, ref) => {
  const { title, description, defaultValue, ...inputProps } = props;

  return (
    <div className="space-y-2">
      <span className="block text-base font-medium">{title}</span>
      <div className="space-y-1">
        <span className="block text-sm text-neutral">{description}</span>
        <span className="block text-sm text-neutral">
          Default value: {defaultValue}
        </span>
      </div>
      <input
        ref={ref}
        className="w-full input input-md input-bordered"
        {...inputProps}
      />
    </div>
  );
});

ConfigInput.displayName = "ConfigInput";
