import { InspectContracts } from "@/components/inspect-contracts";
import { InspectInfra } from "@/components/inspect-infra";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/inspect/")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <div className="max-w-xl mx-auto w-full space-y-10 my-10">
      <h1 className="text-xl font-bold text-center">Manage your chain</h1>

      <div className="space-y-4">
        <InspectContracts />

        <InspectInfra />
      </div>
    </div>
  );
}
