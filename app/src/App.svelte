<script lang="js">
import { invoke } from '@tauri-apps/api';
import { fade, fly, slide, scale } from 'svelte/transition';
import locales from "./locales.json";

import { ScreenLock } from "./screenlock.js";
import { ConfigManager, defaultConfig } from './configmanager.js';
import { Task } from './taskmanager.js';
import { Subject } from './subjects.js';

import Dialog from "./Dialog.svelte";

let creatingTask = false;
let tab = 0;

const screenlock = new ScreenLock();
const conf = new ConfigManager();
const taskmgr = new Task();
const subj = new Subject();

window.subj = subj;

let subjectAdded = false;

$: screenlocked = false;
let cdTimer = {}

let create_subject_name;

//window.locales = locales;

let currentLang;

let timed_lock = false;
let locktime = {};
let lockButtonIsDisabled = false;
let timedLockSwitchIsDisabled = false;


let settings = {};

async function setup() {
	try {
		let lang = await conf.readConfig().misc.lang
	} catch (e) {
		console.log(e);
		await conf.writeConfig(defaultConfig);
		let lang = await conf.readConfig().misc.lang
	}
	
	currentLang = lang;
	try {
		console.log(locales[lang])
		return locales[lang];
	} catch (e) {
		console.log(locales["en"])
		return locales["en"];
	}
}
/*
async function getLangs() {
	let langs = [];
	langs.push( JSON.parse(await conf.readConfig()).misc.lang );
	langs.concat(Object.keys(locales))
	return langs;
}

window.getlangs = getLangs;
*/

function saveSettings() {
	let newSettings = defaultConfig;
	newSettings.misc.lang = settings.lang;
	// Add other settings later

	conf.writeConfig(newSettings);
	window.location.reload();
}

</script>

<main>
	{#await setup()}
	<div class="hero bg-primary text-center">
		<div class="hero-body">
			<div class="loading loading-lg"></div>
		</div>
	</div>
	{:then dict}
	<div class="hero bg-primary text-center">
		<div class="hero-body">
			<h1 is:fade>LibreHomework</h1>
			<p>{dict.summary}</p>
			<!--<p>¿Ponemos algo más aquí? Se siente vacío</p>-->
			<!--<button class="btn" on:click={() => {
				screenlock.Block();
				screenlocked = screenlock.locked;
			}}>{screenlocked ? dict.unlock_screen : dict.lock_screen}</button>-->
		</div>
	</div>
	<svg id="visual" viewBox="0 0 580 95" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1"><path d="M0 90L11.5 90.7C23 91.3 46 92.7 69 92.3C92 92 115 90 138.2 88.2C161.3 86.3 184.7 84.7 207.8 88.7C231 92.7 254 102.3 277 105C300 107.7 323 103.3 346 102.2C369 101 392 103 415.2 103.7C438.3 104.3 461.7 103.7 484.8 102.8C508 102 531 101 554 102.8C577 104.7 600 109.3 623 106.3C646 103.3 669 92.7 692.2 89.8C715.3 87 738.7 92 761.8 95.7C785 99.3 808 101.7 831 102.3C854 103 877 102 888.5 101.5L900 101L900 0L888.5 0C877 0 854 0 831 0C808 0 785 0 761.8 0C738.7 0 715.3 0 692.2 0C669 0 646 0 623 0C600 0 577 0 554 0C531 0 508 0 484.8 0C461.7 0 438.3 0 415.2 0C392 0 369 0 346 0C323 0 300 0 277 0C254 0 231 0 207.8 0C184.7 0 161.3 0 138.2 0C115 0 92 0 69 0C46 0 23 0 11.5 0L0 0Z" fill="#303742"></path><path d="M0 74L11.5 72.8C23 71.7 46 69.3 69 68.7C92 68 115 69 138.2 74.2C161.3 79.3 184.7 88.7 207.8 89.8C231 91 254 84 277 80.5C300 77 323 77 346 75.3C369 73.7 392 70.3 415.2 70.5C438.3 70.7 461.7 74.3 484.8 74.2C508 74 531 70 554 70.2C577 70.3 600 74.7 623 74.5C646 74.3 669 69.7 692.2 67.7C715.3 65.7 738.7 66.3 761.8 67.5C785 68.7 808 70.3 831 74.8C854 79.3 877 86.7 888.5 90.3L900 94L900 0L888.5 0C877 0 854 0 831 0C808 0 785 0 761.8 0C738.7 0 715.3 0 692.2 0C669 0 646 0 623 0C600 0 577 0 554 0C531 0 508 0 484.8 0C461.7 0 438.3 0 415.2 0C392 0 369 0 346 0C323 0 300 0 277 0C254 0 231 0 207.8 0C184.7 0 161.3 0 138.2 0C115 0 92 0 69 0C46 0 23 0 11.5 0L0 0Z" fill="#284676"></path><path d="M0 69L11.5 67.2C23 65.3 46 61.7 69 57.5C92 53.3 115 48.7 138.2 48.2C161.3 47.7 184.7 51.3 207.8 55.7C231 60 254 65 277 64C300 63 323 56 346 55.7C369 55.3 392 61.7 415.2 64.2C438.3 66.7 461.7 65.3 484.8 63C508 60.7 531 57.3 554 54C577 50.7 600 47.3 623 48.5C646 49.7 669 55.3 692.2 59.7C715.3 64 738.7 67 761.8 67.5C785 68 808 66 831 62.2C854 58.3 877 52.7 888.5 49.8L900 47L900 0L888.5 0C877 0 854 0 831 0C808 0 785 0 761.8 0C738.7 0 715.3 0 692.2 0C669 0 646 0 623 0C600 0 577 0 554 0C531 0 508 0 484.8 0C461.7 0 438.3 0 415.2 0C392 0 369 0 346 0C323 0 300 0 277 0C254 0 231 0 207.8 0C184.7 0 161.3 0 138.2 0C115 0 92 0 69 0C46 0 23 0 11.5 0L0 0Z" fill="#2e51aa"></path><path d="M0 28L11.5 29.2C23 30.3 46 32.7 69 34.7C92 36.7 115 38.3 138.2 37.2C161.3 36 184.7 32 207.8 31.7C231 31.3 254 34.7 277 34.8C300 35 323 32 346 32.7C369 33.3 392 37.7 415.2 39.3C438.3 41 461.7 40 484.8 37.5C508 35 531 31 554 31.3C577 31.7 600 36.3 623 38.8C646 41.3 669 41.7 692.2 41.2C715.3 40.7 738.7 39.3 761.8 37.7C785 36 808 34 831 33.5C854 33 877 34 888.5 34.5L900 35L900 0L888.5 0C877 0 854 0 831 0C808 0 785 0 761.8 0C738.7 0 715.3 0 692.2 0C669 0 646 0 623 0C600 0 577 0 554 0C531 0 508 0 484.8 0C461.7 0 438.3 0 415.2 0C392 0 369 0 346 0C323 0 300 0 277 0C254 0 231 0 207.8 0C184.7 0 161.3 0 138.2 0C115 0 92 0 69 0C46 0 23 0 11.5 0L0 0Z" fill="#5755d9"></path></svg>
	<ul class="tab tab-block">
		<li class="tab-item { tab == 0 ? 'active' : ''}">
			<a href="#homework" on:click={() => { tab = 0 }}>{dict.homework}</a>
		</li>
		<li class="tab-item { tab == 1 ? 'active' : ''}">
			<a href="#screenblock" on:click={() => { tab = 1 }}>{dict.screenblock}</a>
		</li>
		<li class="tab-item { tab == 2 ? 'active' : ''}">
			<a href="#subjects" on:click={() => { tab = 2 }}>{dict.subjects}</a>
		</li>
		<li class="tab-item { tab == 3 ? 'active' : ''}">
			<a href="#network" on:click={() => { tab = 3 }}>{dict.network}</a>
		</li>
		<li class="tab-item { tab == 4 ? 'active' : ''}">
			<a href="#settings" on:click={() => { tab = 4 }}>{dict.settings}</a>
		</li>
	</ul>
	
	{#if tab == 0}
	<div class="empty bg-dark pt-3 pb-6" out:fade="{{ duration: 200 }}">
		{#if creatingTask}
			<Dialog on:modalClose={() => { creatingTask = false }} taskmgr={taskmgr} conf={conf} subj={subj} />
		{:else}

			{#await taskmgr.get_batch(10, 0)}
			<div class="loading loading-lg"></div>
			{:then tasks}
				{#if tasks.length > 0}
				<div class="pt-1 pb-2" style="display: flex; justify-content: center;">
					<p class="empty-title h4" style="margin-left: 0; margin-right: auto;" in:slide="{{delay: 100}}">{dict.task_list}</p>
					<button style="margin-right: 1rem;" class="btn btn-primary" on:click={()=>{
						creatingTask = true;
					}}>{dict.add_task}</button>
				</div>

				<table class="table">
					<thead>
						<tr>
      						<th>{dict.name}</th>
							<th>{dict.description }</th>
							<th>{dict.subject}</th>
							<th>{dict.due_date}</th>
							<th>{dict.delete}</th>
						</tr>
					</thead>
					<tbody>
						{#each tasks as task}
						{#if !task.deleted}

						<tr in:fly="{{ delay: 100, x: -200, duration: 800 }}">
							<td>{task.name}</td>
							<td>{task.description || dict.empty}</td>
							<td>{task.subject}</td>
							<td>{new Date(task.expires_at)}</td>
							<td>
								<button class="btn btn-error btn-action" on:click={() => {
									taskmgr.remove(task.id);
									task.deleted = true;
								}}>
									<i class="icon icon-delete"></i>
								</button>
							</td>
						</tr>

						{/if}
						{/each}
					</tbody>
				</table>
				
				{:else}

				<div class="empty-icon" in:slide>
					<i class="icon icon-2x icon-check"></i>
				</div>
				<p class="empty-title h4" in:slide="{{delay: 100}}">{dict.no_activities_left}</p>
				<p class="empty-subtitle">{dict.no_activities_detail}</p>
				<button class="btn btn-primary" on:click={()=>{
					creatingTask = true;
				}}>{dict.add_task}</button>
				{/if}
			{:catch error}
				<div class="empty bg-dark">
					<div class="empty-icon">
						<i class="icon icon-3x icon-cross"></i>
					</div>
					<p class="empty-title h5">{dict.tasks_error}</p>
				</div>
			{/await}

		{/if}
	</div>
	{:else if tab == 1}
	<div class="empty bg-dark pt-3 pb-6" out:fade="{{ duration: 200 }}">
		<div class="empty-icon" in:slide>
			<i class="icon icon-2x icon-shutdown"></i>
		</div>
		<p class="empty-title h3" style="padding-bottom: 1rem; font-weight: 500;">{ (screenlocked ? dict.screen_locked : dict.screen_not_locked).capitalize() }</p>

		<div class="form-group" style="display: flex; justify-content: center; width: fit-content; margin-left: auto; margin-right: auto;">
			<button disabled={lockButtonIsDisabled} class="btn btn-primary" style="margin-right: 1.5rem;" on:click={() => {
				if (!timed_lock) {
					screenlock.Block();
					screenlocked = screenlock.locked;
				} else {		
					if (screenlocked) {
						// Unlock
						screenlock.Block();
						screenlocked = screenlock.locked;
						return;
					}

					// Setup lock wait
					let wait = 0;
					wait += (locktime.seconds || 0);
					wait += (locktime.minutes * 60|| 0);
					wait += ((locktime.hours * 60) * 60 || 0);
					wait = (wait <= 0 ? 5 : wait);

					screenlock.Start(wait);
					lockButtonIsDisabled = true;
					timedLockSwitchIsDisabled = true;	
					screenlocked = screenlock.locked;

					// Setup countdown timer
					let countDownDate = new Date();
					countDownDate.setHours(countDownDate.getHours() + (locktime.hours || 0));
					countDownDate.setMinutes(countDownDate.getMinutes() + (locktime.minutes || 0));
					countDownDate.setSeconds(countDownDate.getSeconds() + (locktime.seconds || 0));

					cdTimer = {};
					let x = setInterval(function() {
						let diff = countDownDate.getTime() - Date.now();
						cdTimer.hours = Math.floor((diff % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
						cdTimer.minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
						cdTimer.seconds = Math.floor((diff % (1000 * 60)) / 1000);

						if (diff < 0) {
							cdTimer = {};
							clearInterval(x);
							screenlocked = false;
							lockButtonIsDisabled = false;
							timedLockSwitchIsDisabled = false;
						}

					}, 1000);
				}
			}}>
				{screenlocked ? dict.unlock_screen : dict.lock_screen}
			</button>
			<label class="form-switch" style="text-align: center; width: fit-content; margin-left: auto; margin-right: auto;">
				<input disabled={timedLockSwitchIsDisabled} type="checkbox" bind:checked={timed_lock}>
					<i class="form-icon"></i> {dict.timed_lock}
			</label>
		</div>
		{#if timed_lock}
			{#if screenlocked}
				<p class="empty-subtitle h2" style="padding-top: 1.5rem;">
					<span>{cdTimer.hours >= 0 ? cdTimer.hours.pad() : '00'}</span> :
					<span>{cdTimer.minutes >= 0 ? cdTimer.minutes.pad() : '00'}</span> :
					<span>{cdTimer.seconds >= 0 ? cdTimer.seconds.pad() : '00'}</span>
				</p>
			{:else}
				<div class="form-group" style="width: fit-content; margin-left: auto; margin-right: auto; padding-top:
				 1rem; ">
				 	<!-- svelte-ignore a11y-label-has-associated-control -->
				 	<label>{dict.about_timed_lock}</label>
					<div class="input-group" style="margin-top: 0.5rem;">
						<input type="number" bind:value={locktime.hours} class="form-input input-group-addon" min="0" placeholder="{dict.hours}">
						<input type="number" bind:value={locktime.minutes} class="form-input" min="0" max="59" placeholder="{dict.minutes}">
						<input type="number" bind:value={locktime.seconds} class="form-input input-group-addon" min="0" max="59" placeholder="{dict.seconds}">
					</div>
				</div>
			{/if}
		{/if}
	</div>
	{:else if tab == 2}
	<div class="empty bg-dark pt-3 pb-6" out:fade="{{ duration: 200 }}">
		<div class="empty-icon" in:slide>
			<i class="icon icon-2x icon-bookmark"></i>
		</div>
		<p class="empty-title h4" in:slide="{{delay: 100}}">{dict.subjects}</p>
		<div class="form-group mt-2">
				<label class="form-label" for="create-subj">{dict.create_subject}</label>
				<div class="input-group">
					<input bind:value={create_subject_name} style="border: #5755d9 3px solid!important; border-right: transparent;" class="form-input" type="text" id="create-subj" placeholder="{dict.math}">
					{#if create_subject_name}
				<button class="btn btn-primary input-group-btn" on:click={()=>{
					subj.create(create_subject_name);
					create_subject_name = "";
					subjectAdded = !subjectAdded;
				}}>{dict.add}</button>
				{:else}
				<button class="btn btn-primary input-group-btn" disabled>{dict.add}</button>
				{/if}
			</div>
			<p class="form-input-hint">{!create_subject_name ? dict.field_empty : ''}</p>
		</div>

		{#key subjectAdded}
		{#await subj.get_batch()}
		<div class="loading loading-lg"></div>
		{:then subs}
			{#if subs.length > 0}
			<table class="table">
				<thead>
					<tr>
						<th>{dict.subject}</th>
						<th>{dict.delete}</th>
					</tr>
				</thead>
				<tbody>
					{#each subs as subject}
					{#if !subject.deleted}

					<tr in:fly="{{ delay: 100, x: -200, duration: 800 }}">
						<td>{subject.name}</td>
						<td>
							<button class="btn btn-error btn-action" on:click={() => {
								subj.remove(subject.id);
								subject.deleted = true;
							}}>
								<i class="icon icon-delete"></i>
							</button>
						</td>
					</tr>

					{/if}
					{/each}
				</tbody>
			</table>
			
			{:else}

			<p class="empty-subtitle">{dict.no_activities_detail}</p>

			{/if}

		{:catch error}
			<div class="empty bg-dark">
				<div class="empty-icon">
					<i class="icon icon-3x icon-cross"></i>
					</div>
				<p class="empty-title h5">{dict.tasks_error}</p>
			</div>
		{/await}
		{/key}
	</div>
	{:else if tab == 3}
	<div>Coming soon...</div>
	{:else if tab == 4}
	<div class="empty bg-dark pt-3 pb-6" out:fade="{{ duration: 200 }}">
		<div class="empty-icon" in:slide>
			<i class="icon icon-2x icon-edit"></i>
		</div>
		<p class="empty-title h4" in:slide="{{delay: 100}}">{dict.settings}</p>
		
		<form class="pt-2" style="width: 60%; margin-left: auto; margin-right: auto;">
			<div class="form-group">
				<label class="form-label" for="language">{dict.language}</label>
				<select class="form-select" style="text-align: center;" id="language" bind:value={settings.lang}>
					<option selected>{currentLang}</option>
					{#each Object.keys(locales) as lang}
					{#if !(lang == currentLang)}
					<option>{lang}</option>
					{/if}
					{/each}
				</select>
			</div>
		</form>

		<button class="btn btn-primary mt-2" on:click={saveSettings}>{dict.save}</button>
	</div>
	{/if}
	
	{:catch error}
		<svg id="visual" viewBox="0 0 580 95" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1"><path d="M0 90L11.5 90.7C23 91.3 46 92.7 69 92.3C92 92 115 90 138.2 88.2C161.3 86.3 184.7 84.7 207.8 88.7C231 92.7 254 102.3 277 105C300 107.7 323 103.3 346 102.2C369 101 392 103 415.2 103.7C438.3 104.3 461.7 103.7 484.8 102.8C508 102 531 101 554 102.8C577 104.7 600 109.3 623 106.3C646 103.3 669 92.7 692.2 89.8C715.3 87 738.7 92 761.8 95.7C785 99.3 808 101.7 831 102.3C854 103 877 102 888.5 101.5L900 101L900 0L888.5 0C877 0 854 0 831 0C808 0 785 0 761.8 0C738.7 0 715.3 0 692.2 0C669 0 646 0 623 0C600 0 577 0 554 0C531 0 508 0 484.8 0C461.7 0 438.3 0 415.2 0C392 0 369 0 346 0C323 0 300 0 277 0C254 0 231 0 207.8 0C184.7 0 161.3 0 138.2 0C115 0 92 0 69 0C46 0 23 0 11.5 0L0 0Z" fill="#303742"></path><path d="M0 74L11.5 72.8C23 71.7 46 69.3 69 68.7C92 68 115 69 138.2 74.2C161.3 79.3 184.7 88.7 207.8 89.8C231 91 254 84 277 80.5C300 77 323 77 346 75.3C369 73.7 392 70.3 415.2 70.5C438.3 70.7 461.7 74.3 484.8 74.2C508 74 531 70 554 70.2C577 70.3 600 74.7 623 74.5C646 74.3 669 69.7 692.2 67.7C715.3 65.7 738.7 66.3 761.8 67.5C785 68.7 808 70.3 831 74.8C854 79.3 877 86.7 888.5 90.3L900 94L900 0L888.5 0C877 0 854 0 831 0C808 0 785 0 761.8 0C738.7 0 715.3 0 692.2 0C669 0 646 0 623 0C600 0 577 0 554 0C531 0 508 0 484.8 0C461.7 0 438.3 0 415.2 0C392 0 369 0 346 0C323 0 300 0 277 0C254 0 231 0 207.8 0C184.7 0 161.3 0 138.2 0C115 0 92 0 69 0C46 0 23 0 11.5 0L0 0Z" fill="#284676"></path><path d="M0 69L11.5 67.2C23 65.3 46 61.7 69 57.5C92 53.3 115 48.7 138.2 48.2C161.3 47.7 184.7 51.3 207.8 55.7C231 60 254 65 277 64C300 63 323 56 346 55.7C369 55.3 392 61.7 415.2 64.2C438.3 66.7 461.7 65.3 484.8 63C508 60.7 531 57.3 554 54C577 50.7 600 47.3 623 48.5C646 49.7 669 55.3 692.2 59.7C715.3 64 738.7 67 761.8 67.5C785 68 808 66 831 62.2C854 58.3 877 52.7 888.5 49.8L900 47L900 0L888.5 0C877 0 854 0 831 0C808 0 785 0 761.8 0C738.7 0 715.3 0 692.2 0C669 0 646 0 623 0C600 0 577 0 554 0C531 0 508 0 484.8 0C461.7 0 438.3 0 415.2 0C392 0 369 0 346 0C323 0 300 0 277 0C254 0 231 0 207.8 0C184.7 0 161.3 0 138.2 0C115 0 92 0 69 0C46 0 23 0 11.5 0L0 0Z" fill="#2e51aa"></path><path d="M0 28L11.5 29.2C23 30.3 46 32.7 69 34.7C92 36.7 115 38.3 138.2 37.2C161.3 36 184.7 32 207.8 31.7C231 31.3 254 34.7 277 34.8C300 35 323 32 346 32.7C369 33.3 392 37.7 415.2 39.3C438.3 41 461.7 40 484.8 37.5C508 35 531 31 554 31.3C577 31.7 600 36.3 623 38.8C646 41.3 669 41.7 692.2 41.2C715.3 40.7 738.7 39.3 761.8 37.7C785 36 808 34 831 33.5C854 33 877 34 888.5 34.5L900 35L900 0L888.5 0C877 0 854 0 831 0C808 0 785 0 761.8 0C738.7 0 715.3 0 692.2 0C669 0 646 0 623 0C600 0 577 0 554 0C531 0 508 0 484.8 0C461.7 0 438.3 0 415.2 0C392 0 369 0 346 0C323 0 300 0 277 0C254 0 231 0 207.8 0C184.7 0 161.3 0 138.2 0C115 0 92 0 69 0C46 0 23 0 11.5 0L0 0Z" fill="#5755d9"></path></svg>
		<div class="empty bg-dark" style="height: 100%;">
			<div class="empty-icon">
				<i class="icon icon-3x icon-cross"></i>
			</div>
			<p class="empty-title h5">Could not load language data</p>
			<p>{error.message}</p>
			<p class="empty-subtitle">Try searching for this issue on our GitHub repo. If you can not find a GitHub issue around this, please create one.</p>
		</div>
	{/await}
</main>

<style>
	main {
		width: 100%;
		height: auto;
		margin: 0 auto;
		padding: 0;
	}

	.hero {
		padding-top: 1rem;
		padding-bottom: 0;
	}

	.tab-item a {
		color: #e5e5f9!important;
	}

	.tab-item .active a {
		color: #fff!important;
	}

	#visual {
		width: 100%;
	}

	#visual > * {
		width: 100%;
	}

	th {
		padding-left: 0;
		font-weight: 500;
	}

	.form-input, .input-group-addon {
		color: white!important;
		background-color: transparent!important;
		border: transparent!important;
		border-bottom-style: solid!important;
		border-bottom-color: #5755d9!important;
		border-bottom-width: 3px!important;
	}

	.form-select {
		color: white!important;
		background-color: transparent!important;
		border: transparent!important;
		border-bottom-style: solid!important;
		border-bottom-color: #5755d9!important;
		border-bottom-width: 3px!important;
	}

	.pt-2 {
		padding-top: 1rem;
	}
</style>
