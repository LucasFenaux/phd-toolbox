export interface AppInfo {
  id: string;
  name: string;
  description: string;
  repo_url: string;
  port: number;
  published_port?: number;
  mode: "prod" | "dev";
  is_installed: boolean;
  is_running: boolean;
  version?: string;
}
