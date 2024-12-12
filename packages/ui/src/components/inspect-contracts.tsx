import axios from "axios";
import { useCallback, useState } from "react";
import { Button } from "./ui";

export const InspectContracts: React.FC = () => {
  const [loading, setLoading] = useState(false);
  const [contractsArtifacts, setContractsArtifacts] = useState(null);
  const [contractsInspection, setContractsInspection] = useState<any>(null);

  const onContractsChange = useCallback(
    (e: any) => {
      setContractsArtifacts(e.target.files[0]);
      setContractsArtifacts(null);
    },
    [setContractsArtifacts]
  );

  const onInspectContracts = useCallback(async (e: any) => {
    e.preventDefault();

    if (!contractsArtifacts) {
      alert("No file selected!");
      return;
    }

    const formData = new FormData();
    formData.append("file", contractsArtifacts);

    try {
      setLoading(true);
      const res = await axios.post("/inspect/contracts", formData, {
        headers: {
          "Content-Type": "multipart/form-data", // Make sure to set the correct header
        },
      });

      setContractsInspection(res.data);
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
          <code>
            /deployments/[name]/artifacts/contracts_artifacts.zip
          </code>
        </p>
      </div>

      <form className="space-y-2" onSubmit={onInspectContracts}>
        <input
          type="file"
          className="file-input block file-input-bordered file-input-sm w-full max-w-xs"
          onChange={onContractsChange}
        />

        <Button className="btn-sm" loading={loading} type="submit">
          Inspect
        </Button>
      </form>

      {contractsInspection && !loading && (
        <div>
          <div>
            <span>Addresses</span>
            <pre>{contractsInspection["addresses"]}</pre>
          </div>

          <div>
            <span>Config</span>
            <pre>{contractsInspection["deploy-config"]}</pre>
          </div>
        </div>
      )}
    </section>
  );
};
