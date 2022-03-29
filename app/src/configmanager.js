import { invoke } from "@tauri-apps/api/tauri";

let syslang = invoke("get_syslang").then(lang => {return lang});

export const defaultConfig = {"misc": {"lang": syslang}, "colors": {"primary": "#3942ed", "secondary": "5056c7"}}

export class ConfigManager {
    async initDefaultConfig() {
        return await invoke("write_config_file", {"contents": JSON.stringify(
            defaultConfig, null, 4)});
    }

    async readConfig() {
        return await invoke("read_config_file");
    }

    async writeConfig(config) {

        return await invoke("write_config_file", {"contents": JSON.stringify(
            config, null, 4)});
    }
}
