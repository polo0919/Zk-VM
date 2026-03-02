import { ZkVMClient } from "@nexus-zkvm/sdk";
import fs from "fs";

async function verifyModelInference() {
    const zkvm = new ZkVMClient({ endpoint: "https://api.zkvm.enterprise.local" });

    // Weights are public, input image is private
    const modelWeights = JSON.parse(fs.readFileSync("resnet_weights.json", "utf-8"));
    const privateImage = "base64_encoded_patient_scan";

    console.log("Submitting ML inference to zkVM Execution Engine...");

    const session = await zkvm.execute({
        programId: "resnet50_verified_inference",
        privateInput: { image: privateImage },
        publicInput: { weightsHash: modelWeights.hash }
    });

    console.log("Execution complete. Requesting STARK proof...");
    const proof = await zkvm.prove(session.id);

    console.log(`Proof generated: ${proof.id}. Verifying...`);
    const isValid = await zkvm.verify(proof);

    if (isValid) {
        console.log("✅ Diagnosis verified successfully without revealing patient data.");
    } else {
        console.error("❌ Invalid proof.");
    }
}

verifyModelInference().catch(console.error);
