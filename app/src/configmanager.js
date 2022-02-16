import { tauri } from "@tauri-apps/api";

class ConfigManager {
    initDefaultConfig() {
        tauri.fs.writeFile("$DOCUMENT/LibreHomework/config.json", JSON.stringify(
            { "misc": {"lang": "en"}, "colors": {"primary": "#3942ed", "secondary": "5056c7"}}), (err) => {
                if (err) {
                    return err;
                }
            });
    }

    readConfig() {
        tauri.fs.readTextFile("$DOCUMENT/LibreHomework/config.json", (err, data) => {
            if (err) {
                return error;
            } else {
                return JSON.parse(data);
            }
        });
    }
}

export default ConfigManager;