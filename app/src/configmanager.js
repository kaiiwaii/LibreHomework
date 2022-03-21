import { invoke } from "@tauri-apps/api/tauri";

export const defaultConfig = { "misc": {"lang": "es"}, "colors": {"primary": "#3942ed", "secondary": "5056c7"}}

export class ConfigManager {
    async initDefaultConfig() {
        return await invoke("write_config_file", {"contents": JSON.stringify(
            defaultConfig, null, 4)});
    }

    async readConfig() {
        return await invoke("read_config_file");
    }

    async writeConfig(config) {
        // Actually this is not very safe, it might lead to a little self-XSS vuln, but anyway... 😳
        return await invoke("write_config_file", {"contents": JSON.stringify(
            config, null, 4)});
    }
}
