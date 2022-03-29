import { app, invoke } from '@tauri-apps/api';
import { ConfigManager } from "./configmanager.js";
//import { ScreenLock } from './screenlock';
import App from './App.svelte';
import ServerAPI from "./network.js"

const svapp = new App({
	target: document.body
});

let v = app.getVersion();


let conf = new ConfigManager();
conf.readConfig().then((v) => {
	if (v == null) {
		conf.initDefaultConfig();		
	}
});


Number.prototype.pad = function(pad) {
	let p = Math.pow(10, pad || 2);
	let a = Math.abs(this);
	let g = (this<0);
	return (a < p) ?  ((g ? '-' : '') + (p+a).toString().substring(1)) : this;
}

String.prototype.capitalize = function() {
	let newString = "";
	for (let w of this.split(" ")) {
		newString += (w[0].toUpperCase() + w.substring(1)) + " ";
	}
	return newString;
}


export default svapp;
