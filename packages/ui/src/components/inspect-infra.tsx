import axios from "axios";
import { useCallback, useState } from "react";
import { Button } from "./ui";

export const InspectInfra: React.FC = () => {
  const [loading, setLoading] = useState(false);
  const [infraArtifacts, setInfraArtifacts] = useState(null);
  const [infraInspection, setInfraInspection] = useState<any>(null);

  const onInfraChange = useCallback(
    (e: any) => {
      setInfraArtifacts(e.target.files[0]);
      setInfraArtifacts(null);
    },
    [setInfraArtifacts]
  );

  const onInspectInfra = useCallback(async (e: any) => {
    e.preventDefault();

    if (!infraArtifacts) {
      alert("No file selected!");
      return;
    }

    const formData = new FormData();
    formData.append("file", infraArtifacts);

    try {
      setLoading(true);
      const res = await axios.post("/inspect/infra", formData, {
        headers: {
          "Content-Type": "multipart/form-data", // Make sure to set the correct header
        },
      });

      setInfraInspection(res.data);
    } catch (e) {
      window.alert(e);
    } finally {
      setLoading(false);
    }
  }, []);

  return (
    <section className="space-y-4 border bg-gray-100 p-3 rounded-md text-sm">
      <div className="space-y-2">
        <h2 className="font-medium">Inspect contracts deployments</h2>
        <p>
          Find it at{" "}
          <code>/deployments/[name]/artifacts/infra_artifacts.zip</code>
        </p>
      </div>

      <form className="space-y-2" onSubmit={onInspectInfra}>
        <input
          type="file"
          className="file-input block file-input-bordered file-input-sm w-full max-w-xs"
          onChange={onInfraChange}
        />

        <Button className="btn-sm" loading={loading} type="submit">
          Inspect
        </Button>
      </form>

      {infraInspection && !loading && (
        <div>
          <div>
            <span>Outputs</span>
            <pre>{infraInspection["outputs"]}</pre>
          </div>
        </div>
      )}
    </section>
  );
};
