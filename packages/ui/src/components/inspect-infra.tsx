import axios from "axios";
import { useCallback, useState } from "react";

export const InspectInfra: React.FC = () => {
  const [loading, setLoading] = useState(false);
  const [infraInspection, setInfraInspection] = useState<any>(null);

  const onInfraChange = useCallback(
    async (e: any) => {
      setInfraInspection(null);

      const formData = new FormData();
      formData.append("file", e.target.files[0]);

      try {
        setLoading(true);
        const res = await axios.post(
          import.meta.env.VITE_SERVER_URL + "/inspect/infra",
          formData,
          {
            headers: {
              "Content-Type": "multipart/form-data", // Make sure to set the correct header
            },
          }
        );

        setInfraInspection(res.data);
      } catch (e) {
        window.alert(e);
      } finally {
        setLoading(false);
      }
    },
    [setInfraInspection]
  );

  return (
    <section className="space-y-4 border bg-gray-100 p-3 rounded-md text-sm">
      <div className="space-y-2">
        <h2 className="font-medium">Inspect infra deployments</h2>
        <p>
          Find it at{" "}
          <code>/deployments/[name]/artifacts/infra_artifacts.zip</code>
        </p>
      </div>

      <input
        type="file"
        className="file-input block file-input-bordered file-input-sm w-full max-w-xs"
        onChange={onInfraChange}
      />

      {infraInspection && !loading && (
        <div className="border-t mt-4 pt-4">
          <div className="space-y-2">
            <span className="font-medium">Outputs</span>
            <pre className="border rounded-md p-2 overflow-x-scroll">
              {JSON.stringify(infraInspection["outputs"], null, 2)}
            </pre>
          </div>
        </div>
      )}
    </section>
  );
};
