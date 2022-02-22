/*import { tauri } from "@tauri-apps/api";*/
import { writeFile, readTextFile } from '@tauri-apps/api/fs';

export class ConfigManager {
    initDefaultConfig() {
        writeFile("$DOCUMENT/LibreHomework/config.json", JSON.stringify(
            { "misc": {"lang": "en"}, "colors": {"primary": "#3942ed", "secondary": "5056c7"}}), (err) => {
                if (err) {
                    return err;
                }
            });
    }

    readConfig() {
        readTextFile("$DOCUMENT/LibreHomework/config.json", (err, data) => {
            if (err) {
                return error;
            } else {
                return JSON.parse(data);
            }
        });
    }
}
