import { invoke } from "@tauri-apps/api/tauri";

export class ConfigManager {
    async initDefaultConfig() {
        return await invoke("write_config_file", {"contents": JSON.stringify(
            { "misc": {"lang": "en"}, "colors": {"primary": "#3942ed", "secondary": "5056c7"}}, null, 4)});
    }

    async readConfig() {
        return await invoke("read_config_file");
    }
}
