<script lang="js">
import { invoke } from '@tauri-apps/api';
import { ScreenLock } from "./screenlock.js";
import { ConfigManager } from './configmanager.js';
import locales from "./locales.json";

export let screenlock = new ScreenLock();
const conf = new ConfigManager();

let taskData = {};

function addTask() {
	datetime = taskData.date
	taskData = {}
}

async function getTasks(page = 0) {
	// Limit set to 100, will implement pages later
	return await invoke("getTasks", { limit: 100, page: page });
}

async function setup() {
	let lang = JSON.parse(await conf.readConfig()).misc.lang || "es";
	return locales[lang];
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
			<h1>LibreHomework</h1>
			<p>{dict.summary}</p>
			<button class="btn" on:click={screenlock.Block}>{dict.lock_screen}</button>
			<!--
			<div class="container">
				<div class="columns">
					<div class="col-3"></div>
						<div class="col-6"><div class="form-group">
						  <label class="form-checkbox">
						    <input type="checkbox">
						    <i class="form-icon"></i>{dict.timed_lock}
						  </label>
						</div>
					</div>
				</div>
			</div>
			-->
		</div>
	</div>
	<svg id="visual" viewBox="0 0 580 95" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1"><path d="M0 90L11.5 90.7C23 91.3 46 92.7 69 92.3C92 92 115 90 138.2 88.2C161.3 86.3 184.7 84.7 207.8 88.7C231 92.7 254 102.3 277 105C300 107.7 323 103.3 346 102.2C369 101 392 103 415.2 103.7C438.3 104.3 461.7 103.7 484.8 102.8C508 102 531 101 554 102.8C577 104.7 600 109.3 623 106.3C646 103.3 669 92.7 692.2 89.8C715.3 87 738.7 92 761.8 95.7C785 99.3 808 101.7 831 102.3C854 103 877 102 888.5 101.5L900 101L900 0L888.5 0C877 0 854 0 831 0C808 0 785 0 761.8 0C738.7 0 715.3 0 692.2 0C669 0 646 0 623 0C600 0 577 0 554 0C531 0 508 0 484.8 0C461.7 0 438.3 0 415.2 0C392 0 369 0 346 0C323 0 300 0 277 0C254 0 231 0 207.8 0C184.7 0 161.3 0 138.2 0C115 0 92 0 69 0C46 0 23 0 11.5 0L0 0Z" fill="#303742"></path><path d="M0 74L11.5 72.8C23 71.7 46 69.3 69 68.7C92 68 115 69 138.2 74.2C161.3 79.3 184.7 88.7 207.8 89.8C231 91 254 84 277 80.5C300 77 323 77 346 75.3C369 73.7 392 70.3 415.2 70.5C438.3 70.7 461.7 74.3 484.8 74.2C508 74 531 70 554 70.2C577 70.3 600 74.7 623 74.5C646 74.3 669 69.7 692.2 67.7C715.3 65.7 738.7 66.3 761.8 67.5C785 68.7 808 70.3 831 74.8C854 79.3 877 86.7 888.5 90.3L900 94L900 0L888.5 0C877 0 854 0 831 0C808 0 785 0 761.8 0C738.7 0 715.3 0 692.2 0C669 0 646 0 623 0C600 0 577 0 554 0C531 0 508 0 484.8 0C461.7 0 438.3 0 415.2 0C392 0 369 0 346 0C323 0 300 0 277 0C254 0 231 0 207.8 0C184.7 0 161.3 0 138.2 0C115 0 92 0 69 0C46 0 23 0 11.5 0L0 0Z" fill="#284676"></path><path d="M0 69L11.5 67.2C23 65.3 46 61.7 69 57.5C92 53.3 115 48.7 138.2 48.2C161.3 47.7 184.7 51.3 207.8 55.7C231 60 254 65 277 64C300 63 323 56 346 55.7C369 55.3 392 61.7 415.2 64.2C438.3 66.7 461.7 65.3 484.8 63C508 60.7 531 57.3 554 54C577 50.7 600 47.3 623 48.5C646 49.7 669 55.3 692.2 59.7C715.3 64 738.7 67 761.8 67.5C785 68 808 66 831 62.2C854 58.3 877 52.7 888.5 49.8L900 47L900 0L888.5 0C877 0 854 0 831 0C808 0 785 0 761.8 0C738.7 0 715.3 0 692.2 0C669 0 646 0 623 0C600 0 577 0 554 0C531 0 508 0 484.8 0C461.7 0 438.3 0 415.2 0C392 0 369 0 346 0C323 0 300 0 277 0C254 0 231 0 207.8 0C184.7 0 161.3 0 138.2 0C115 0 92 0 69 0C46 0 23 0 11.5 0L0 0Z" fill="#2e51aa"></path><path d="M0 28L11.5 29.2C23 30.3 46 32.7 69 34.7C92 36.7 115 38.3 138.2 37.2C161.3 36 184.7 32 207.8 31.7C231 31.3 254 34.7 277 34.8C300 35 323 32 346 32.7C369 33.3 392 37.7 415.2 39.3C438.3 41 461.7 40 484.8 37.5C508 35 531 31 554 31.3C577 31.7 600 36.3 623 38.8C646 41.3 669 41.7 692.2 41.2C715.3 40.7 738.7 39.3 761.8 37.7C785 36 808 34 831 33.5C854 33 877 34 888.5 34.5L900 35L900 0L888.5 0C877 0 854 0 831 0C808 0 785 0 761.8 0C738.7 0 715.3 0 692.2 0C669 0 646 0 623 0C600 0 577 0 554 0C531 0 508 0 484.8 0C461.7 0 438.3 0 415.2 0C392 0 369 0 346 0C323 0 300 0 277 0C254 0 231 0 207.8 0C184.7 0 161.3 0 138.2 0C115 0 92 0 69 0C46 0 23 0 11.5 0L0 0Z" fill="#5755d9"></path></svg>
	<ul class="tab tab-block">
	  <li class="tab-item active">
	    <a href="#homework">{dict.homework}</a>
	  </li>
	  <li class="tab-item">
	    <a href="#schedule">{dict.schedule}</a>
	  </li>
	  <li class="tab-item">
	    <a href="#organization">{dict.organization}</a>
	  </li>
	  <li class="tab-item">
	    <a href="#settings">{dict.settings}</a>
	  </li>
	</ul>
	
	<div class="empty bg-dark">
		{#if taskData.editing}
			<form>
				<div class="form-group">
					<label class="form-label" for="name">{dict.task_name}</label>
					<input bind:value={taskData.name} class="form-input" id="name" type="text" placeholder={dict.write_essay}>
				</div>
				<div class="form-group">
					<label class="form-label" for="description">{dict.task_description}</label>
					<textarea bind:value={taskData.desc} class="form-input" id="description" placeholder={dict.essay_about_edu}></textarea>
				</div>
				<div class="form-group">
					<label class="form-label" for="due_date">{dict.due_date}</label>
					<input bind:value={taskData.date} class="form-input" id="due_date" type="date">
				</div>
			</form>
		{:else}
			{#await getTasks()}
			<p>Loading tasks</p>
			{:then tasks}
				{#if tasks}
				{tasks}
				<div class="empty-icon">
					<i class="icon icon-2x icon-check"></i>
				</div>
				<p class="empty-title h4">{dict.no_activities_left}</p>
				<p class="empty-subtitle">{dict.no_activities_detail}</p>
				{:else}
				{tasks}
				{/if}
			{:catch error}
				<div class="empty bg-dark">
					<div class="empty-icon">
						<i class="icon icon-3x icon-cross"></i>
					</div>
					<p class="empty-title h5">Could not load tasks</p>
					<p class="empty-subtitle">Try searching for this issue on our GitHub repo. If you can not find a GitHub issue around this, please create one.</p>
				</div>
			{/await}
			
			<button class="btn btn-primary" on:click={()=>{
				taskData.editing = true;
			}}>{dict.add_task}</button>
		{/if}
	</div>
	<!-- svelte-ignore a11y-missing-content -->
	<div class="modal modal-sm" id="add-task-modal"><a class="modal-overlay" href="#modals-sizes" aria-label="Close"></a>
	 <div class="modal-container bg-dark">
	   <div class="modal-header" style="padding-bottom:0.5rem;">
         <a class="btn btn-clear float-right" href="#modals-sizes" aria-label="Close" style="color: #fff" on:click={()=>{
			document.querySelector("#add-task-modal").classList.remove("active")
		 }}></a>
	     <div class="modal-title h5" style="color: #fff">{dict.add_task}</div>
	   </div>
	   <div class="modal-body" style="padding-top:0.5rem;">
	     <div class="content">
	       
	     </div>
	   </div>
	   <div class="modal-footer">
	     <button class="btn btn-primary" on:click={addTask}>{dict.add}</button>
	     <a class="btn btn-link" href="#modals-sizes" aria-label="Close" style="color: #fff" on:click={()=>{
			document.querySelector("#add-task-modal").classList.remove("active")
		  }}>{dict.close}</a>
	   </div>
	 </div>
	</div>
	{:catch error}
		<svg id="visual" viewBox="0 0 580 95" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1"><path d="M0 90L11.5 90.7C23 91.3 46 92.7 69 92.3C92 92 115 90 138.2 88.2C161.3 86.3 184.7 84.7 207.8 88.7C231 92.7 254 102.3 277 105C300 107.7 323 103.3 346 102.2C369 101 392 103 415.2 103.7C438.3 104.3 461.7 103.7 484.8 102.8C508 102 531 101 554 102.8C577 104.7 600 109.3 623 106.3C646 103.3 669 92.7 692.2 89.8C715.3 87 738.7 92 761.8 95.7C785 99.3 808 101.7 831 102.3C854 103 877 102 888.5 101.5L900 101L900 0L888.5 0C877 0 854 0 831 0C808 0 785 0 761.8 0C738.7 0 715.3 0 692.2 0C669 0 646 0 623 0C600 0 577 0 554 0C531 0 508 0 484.8 0C461.7 0 438.3 0 415.2 0C392 0 369 0 346 0C323 0 300 0 277 0C254 0 231 0 207.8 0C184.7 0 161.3 0 138.2 0C115 0 92 0 69 0C46 0 23 0 11.5 0L0 0Z" fill="#303742"></path><path d="M0 74L11.5 72.8C23 71.7 46 69.3 69 68.7C92 68 115 69 138.2 74.2C161.3 79.3 184.7 88.7 207.8 89.8C231 91 254 84 277 80.5C300 77 323 77 346 75.3C369 73.7 392 70.3 415.2 70.5C438.3 70.7 461.7 74.3 484.8 74.2C508 74 531 70 554 70.2C577 70.3 600 74.7 623 74.5C646 74.3 669 69.7 692.2 67.7C715.3 65.7 738.7 66.3 761.8 67.5C785 68.7 808 70.3 831 74.8C854 79.3 877 86.7 888.5 90.3L900 94L900 0L888.5 0C877 0 854 0 831 0C808 0 785 0 761.8 0C738.7 0 715.3 0 692.2 0C669 0 646 0 623 0C600 0 577 0 554 0C531 0 508 0 484.8 0C461.7 0 438.3 0 415.2 0C392 0 369 0 346 0C323 0 300 0 277 0C254 0 231 0 207.8 0C184.7 0 161.3 0 138.2 0C115 0 92 0 69 0C46 0 23 0 11.5 0L0 0Z" fill="#284676"></path><path d="M0 69L11.5 67.2C23 65.3 46 61.7 69 57.5C92 53.3 115 48.7 138.2 48.2C161.3 47.7 184.7 51.3 207.8 55.7C231 60 254 65 277 64C300 63 323 56 346 55.7C369 55.3 392 61.7 415.2 64.2C438.3 66.7 461.7 65.3 484.8 63C508 60.7 531 57.3 554 54C577 50.7 600 47.3 623 48.5C646 49.7 669 55.3 692.2 59.7C715.3 64 738.7 67 761.8 67.5C785 68 808 66 831 62.2C854 58.3 877 52.7 888.5 49.8L900 47L900 0L888.5 0C877 0 854 0 831 0C808 0 785 0 761.8 0C738.7 0 715.3 0 692.2 0C669 0 646 0 623 0C600 0 577 0 554 0C531 0 508 0 484.8 0C461.7 0 438.3 0 415.2 0C392 0 369 0 346 0C323 0 300 0 277 0C254 0 231 0 207.8 0C184.7 0 161.3 0 138.2 0C115 0 92 0 69 0C46 0 23 0 11.5 0L0 0Z" fill="#2e51aa"></path><path d="M0 28L11.5 29.2C23 30.3 46 32.7 69 34.7C92 36.7 115 38.3 138.2 37.2C161.3 36 184.7 32 207.8 31.7C231 31.3 254 34.7 277 34.8C300 35 323 32 346 32.7C369 33.3 392 37.7 415.2 39.3C438.3 41 461.7 40 484.8 37.5C508 35 531 31 554 31.3C577 31.7 600 36.3 623 38.8C646 41.3 669 41.7 692.2 41.2C715.3 40.7 738.7 39.3 761.8 37.7C785 36 808 34 831 33.5C854 33 877 34 888.5 34.5L900 35L900 0L888.5 0C877 0 854 0 831 0C808 0 785 0 761.8 0C738.7 0 715.3 0 692.2 0C669 0 646 0 623 0C600 0 577 0 554 0C531 0 508 0 484.8 0C461.7 0 438.3 0 415.2 0C392 0 369 0 346 0C323 0 300 0 277 0C254 0 231 0 207.8 0C184.7 0 161.3 0 138.2 0C115 0 92 0 69 0C46 0 23 0 11.5 0L0 0Z" fill="#5755d9"></path></svg>
		<div class="empty bg-dark" style="height: 100%;">
			<div class="empty-icon">
				<i class="icon icon-3x icon-cross"></i>
			</div>
			<p class="empty-title h5">Could not load language data</p>
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
</style>
