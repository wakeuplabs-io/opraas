import { Command } from '@/components/command'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/deploy/')({
  component: DeployChain,
})

function DeployChain() {
  return (
    <div className="max-w-xl mx-auto w-full space-y-10 my-10">
      <h1 className="text-xl font-bold text-center">Deploy your chain locally</h1>

      <div className="space-y-4 text-sm">
        <section className="space-y-2 border bg-gray-100 p-3 rounded-md">
          <h2 className="font-medium">Install opruaas</h2>

          <ol className="space-y-2">
            <li className="space-y-2">
              <div>Install opruaas cli with</div>
              <Command command="npm i -g @wakeuplabs/opruaas" />
            </li>
            <li>
              <div>Install dependencies if needed</div>
              <ul className="list-disc pl-4 mt-2">
                <li>Docker</li>
                <li>Kubernettes</li>
                <li>Helm</li>
              </ul>
            </li>
          </ol>
        </section>

        <section className="space-y-2 border bg-gray-100 p-3 rounded-md">
          <h2 className="font-medium">Run in dev mode</h2>

          <ol>
            <li className="space-y-2">
              <div>Run from inside the project directory</div>
              <Command command="npx opruaas dev --default" />
            </li>
          </ol>
        </section>

        <section className="space-y-2 border bg-gray-100 p-3 rounded-md">
          <h2 className="font-medium">Deploy!</h2>

          <ol>
            <li className="space-y-2">
              <div>Run from inside the project directory</div>
              <Command command="opruaas deploy --name prod --target all" />
            </li>
          </ol>
        </section>
      </div>
    </div>
  )
}
