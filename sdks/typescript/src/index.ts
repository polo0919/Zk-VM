import axios, { AxiosInstance } from 'axios';

export interface ZkVMClientOptions {
    endpoint: string;
    apiKey?: string;
}

export interface CompileResult {
    bytecodeId: string;
    hash: string;
}

export interface ExecuteParams {
    programId: string;
    privateInput?: Record<string, any>;
    publicInput?: Record<string, any>;
}

export interface Session {
    id: string;
    traceId: string;
    status: 'COMPLETED' | 'FAILED';
    publicOutputs: Record<string, any>;
}

export interface Proof {
    id: string;
    traceCommitment: string;
    proofData: string;
    publicInputs: number[];
}

export class ZkVMClient {
    private api: AxiosInstance;

    constructor(options: ZkVMClientOptions) {
        this.api = axios.create({
            baseURL: options.endpoint,
            headers: {
                ...(options.apiKey && { 'Authorization': `Bearer ${options.apiKey}` }),
                'Content-Type': 'application/json'
            }
        });
    }

    /**
     * Compiles high-level code into zkVM bytecode
     */
    async compile(sourceFile: string, code: string): Promise<CompileResult> {
        const response = await this.api.post('/compile', { filename: sourceFile, source: code });
        return response.data;
    }

    /**
     * Executes the compiled bytecode and generates an execution trace
     */
    async execute(params: ExecuteParams): Promise<Session> {
        const response = await this.api.post('/execute', params);
        return response.data;
    }

    /**
     * Submits an execution trace to the distributed proving cluster
     */
    async prove(sessionId: string): Promise<Proof> {
        const response = await this.api.post(`/prove`, { sessionId });
        return response.data;
    }

    /**
     * Verifies a STARK/SNARK proof cryptographically
     */
    async verify(proof: Proof): Promise<boolean> {
        const response = await this.api.post('/verify', { proof });
        return response.data.valid;
    }
}

export const Compiler = {
    compile: async (filePath: string): Promise<string> => {
        // In a real implementation this would invoke the local rust compiler via FFI or WASM
        // For now we return a mock bytecode string
        return `mock_bytecode_for_${filePath}`;
    }
};
