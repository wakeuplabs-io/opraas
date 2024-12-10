export const ConfigSection = (props: {
  title: string;
  description?: string;
  children: React.ReactNode;
}) => {
  return (
    <section className="space-y-6 py-6">
      <div className="space-y-2">
        <h3 className="text-lg font-semibold">{props.title}</h3>
        {props.description && <p className="text-sm">{props.description}</p>}
      </div>

      <div className="space-y-4">{props.children}</div>
    </section>
  );
};
