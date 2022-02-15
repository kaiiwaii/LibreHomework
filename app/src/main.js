import { app, invoke} from '@tauri-apps/api';
//import { ScreenLock } from './screenlock';
import App from './App.svelte';



const svapp = new App({
	target: document.body
});

let v = app.getVersion();

//invoke("execute", {query: "CREATE TABLE IF NOT EXISTS test (name TEXT);"});

//let win = appWindow.label==null ? console.log("appWindow null") : console.log("appWindow is not null");
//console.log(appWindow.label)
//let lock = new ScreenLock().Block()

//invoke("get_local_lang").then((v) => console.log(v));
invoke("notify", {title: "LibreHomework", message: "LibreHomework is running"}).then(() => console.log("notify"));

export default svapp;
