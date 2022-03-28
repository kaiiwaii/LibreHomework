import { app, invoke } from '@tauri-apps/api';
import { ConfigManager } from "./configmanager.js";
//import { ScreenLock } from './screenlock';
import App from './App.svelte';
import ServerAPI from "./network.js"

const svapp = new App({
	target: document.body
});

let v = app.getVersion();

//invoke("execute", {query: "CREATE TABLE IF NOT EXISTS test (name TEXT);"});

//let win = appWindow.label==null ? console.log("appWindow null") : console.log("appWindow is not null");
//console.log(appWindow.label)
//let lock = new ScreenLock().Block()

//invoke("get_local_lang").then((v) => console.log(v));

//invoke("notify", {title: "LibreHomework", message: "LibreHomework is running"}).then(() => console.log("notify"));

let conf = new ConfigManager();
conf.readConfig().then((v) => {
	if (v == null) {
		conf.initDefaultConfig();		
	}
});

//conf.initDefaultConfig()//.then(()=>{}).catch(err => console.log(err));
//conf.readConfig().then((v) => console.log(v));
/*
invoke("getTasks", {limit: 10, page: 0}).then(t => {
	console.log(t)
});*/

Number.prototype.pad = function(pad) {
	let p = Math.pow(10, pad || 2);
	let a = Math.abs(this);
	let g = (this<0);
	return (a < p) ?  ((g ? '-' : '') + (p+a).toString().substring(1)) : this;
}

// I know that this is probably unoptimized as... but anyways... 😳
String.prototype.capitalize = function() {
	let newString = "";
	for (let w of this.split(" ")) {
		newString += (w[0].toUpperCase() + w.substring(1)) + " ";
	}
	return newString;
}

let net = new ServerAPI()
console.log(net)
net.getDailyMessage().then((d) => console.log(d))
net.login("username", "password").then((d) => console.log(d))

export default svapp;
