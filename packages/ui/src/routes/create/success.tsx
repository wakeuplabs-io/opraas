import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/create/success")({
  component: CreateChain,
});

function CreateChain() {
  return (
    <div className="max-w-xl mx-auto w-full space-y-10 my-10">
      <h1 className="text-xl font-bold text-center">What's next?</h1>

      <div className="space-y-4 text-sm">
        <section className="space-y-2 border bg-gray-100 p-3 rounded-md">
          <h2 className="font-medium">Install opruaas</h2>

          <ol>
            <li>
              Install opruaas cli with <code>npm i -g @wakeuplabs/opruaas</code>
            </li>
            <li>
              <span>Install dependencies if needed</span>
              <ul className="list-disc pl-4">
                <li>Kubernettes</li>
              </ul>
            </li>
          </ol>
        </section>

        <section className="space-y-2 border bg-gray-100 p-3 rounded-md">
          <h2 className="font-medium">Run in dev mode</h2>

          <ol>
            <li>
              Run{" "}
              <code>npx opruaas dev --default</code>
              from inside the project directory
            </li>
          </ol>
        </section>

        <section className="space-y-2 border bg-gray-100 p-3 rounded-md">
          <h2 className="font-medium">Deploy!</h2>

          <ol>
            <li>
              Run{" "}
              <code>opruaas deploy --name prod --target all</code>
              from inside the project directory
            </li>
          </ol>
        </section>
      </div>

      <div className="text-center space-x-2">
        <button className="btn btn-sm">Back</button>
        <button className="btn btn-sm">Download</button>
      </div>
    </div>
  );
}
