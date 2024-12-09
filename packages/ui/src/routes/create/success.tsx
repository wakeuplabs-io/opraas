import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/create/success')({
  component: CreateChain,
})

function CreateChain() {
  return (
    <div className="max-w-xl mx-auto w-full space-y-10 my-10">
      <h1 className="text-xl font-bold">What's next?</h1>

      <ol>
        <li>1. Instal opruaas cli with <code>npm i -g @wakeuplabs/opruaas</code></li>
        <li>2. cd into the downloaded zip</li>
        <li>3. Run <code>npx opruaas dev --default</code> if you wish to test your chain locally</li>
        <li>4. Run <code>npx opruaas deploy all --name prod</code> to deploy your chain</li>
      </ol>

      <div>
        <span>Note: Make sure to install opruaas dependencies as well. Find more in the documentation here.</span>
      </div>
    </div>
  )
}
