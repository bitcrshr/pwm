declare module "bun" {
  interface Env {
    LINODE_S3_ACCESS_KEY: string;
    LINODE_S3_SECRET_KEY: string;
  }
}
