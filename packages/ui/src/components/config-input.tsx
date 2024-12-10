import React, { forwardRef } from "react";

export const ConfigInput = forwardRef<
  HTMLInputElement,
  {
    title: string;
    description: string;
    defaultValue?: string;
    type?: string;
    recommendedValue?: string;
    notes?: string;
    standardConfigRequirement?: string;
  } & React.InputHTMLAttributes<HTMLInputElement>
>((props, ref) => {
  const {
    title,
    description,
    defaultValue,
    type,
    recommendedValue,
    notes,
    standardConfigRequirement,
    ...inputProps
  } = props;

  return (
    <div className="space-y-3 py-2">
      <span className="block text-base font-medium">{title}</span>
      <div className="space-y-1">
        <span className="block text-sm text-neutral">{description}</span>
        <ul className="list-disc pl-4">
          {type && (
            <li className="text-sm text-neutral space-x-2">
              <span className="font-semibold">Type:</span>
              <span>{type}</span>
            </li>
          )}
          {defaultValue && (
            <li className="text-sm text-neutral space-x-2">
              <span className="font-semibold">Default value:</span>
              <span>{defaultValue}</span>
            </li>
          )}
          {recommendedValue && (
            <li className="text-sm text-neutral space-x-2">
              <span className="font-semibold">Recommended value:</span>
              <span>{recommendedValue}</span>
            </li>
          )}
          {notes && (
            <li className="text-sm text-neutral space-x-2">
              <span className="font-semibold">Notes:</span>
              <span>{notes}</span>
            </li>
          )}
          {standardConfigRequirement && (
            <li className="text-sm text-neutral space-x-2">
              <span className="font-semibold">Standard config requirement:</span>
              <span>{standardConfigRequirement}</span>
            </li>
          )}
        </ul>
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
