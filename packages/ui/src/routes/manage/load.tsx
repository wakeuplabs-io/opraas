import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/manage/index copy')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div>
      <h1>Manage your chain</h1>

      <section>
        <h2>Load your chain</h2>
        <div>Drop area</div>
      </section>
    </div>
  )
}
