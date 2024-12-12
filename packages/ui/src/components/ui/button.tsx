import { cn } from "@/lib/utils";

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  loading: boolean;
}

export function Button(props: ButtonProps) {
  const { disabled, loading, className, ...rest } = props;
  return (
    <button
      disabled={disabled || loading}
      className={cn("btn", className)}
      {...rest}
    />
  );
}
