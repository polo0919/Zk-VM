provider "aws" {
  region = "us-east-1"
}

resource "aws_eks_cluster" "zkvm_cluster" {
  name     = "zkvm-prover-cluster"
  role_arn = aws_iam_role.cluster.arn

  vpc_config {
    subnet_ids = aws_subnet.public[*].id
  }
}

resource "aws_eks_node_group" "gpu_provers" {
  cluster_name    = aws_eks_cluster.zkvm_cluster.name
  node_group_name = "gpu-prover-nodes"
  node_role_arn   = aws_iam_role.node.arn
  subnet_ids      = aws_subnet.private[*].id
  instance_types  = ["g4dn.xlarge"] # GPU Instances
  
  scaling_config {
    desired_size = 2
    max_size     = 50
    min_size     = 1
  }
}

resource "aws_s3_bucket" "execution_traces" {
  bucket = "zkvm-execution-traces"
}

resource "aws_s3_bucket" "proof_artifacts" {
  bucket = "zkvm-proof-artifacts"
}
