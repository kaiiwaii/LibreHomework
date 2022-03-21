<script>
	import { fade } from 'svelte/transition';
	import { createEventDispatcher } from 'svelte';

	import locales from "./locales.json";

	export let taskmgr;
	export let conf;
	export let subj;

	const dispatch = createEventDispatcher();

	let taskData = {
		time: "23:59"
	}

	let dateError = "";
	let create_subject_name;

	function getMinDate() {
		let mindate = new Date(Date.now());
		return `${mindate.getFullYear()}-${(mindate.getMonth()+1).pad()}-${mindate.getDate().pad()}`
	}

	async function setup() {
		let lang = JSON.parse(await conf.readConfig()).misc.lang || "es";
		return locales[lang];
	}

	async function setupSubs() {
		let subs = await subj.get_batch()
		return subs;
	}

	function addTask() {
		if (!taskData.name && !taskData.subject && !taskData.date && !taskData.time) return;

		let tempDate = taskData.jsDate.valueAsDate;
		tempDate.setDate(tempDate.getDate() + 1);

		tempDate.setHours( taskData.time.match( /(\d{2}):\d{2}/ )[1] );
		tempDate.setMinutes( taskData.time.match( /\d{2}:(\d{2})/ )[1] );

		if (tempDate.getTime() < Date.now()) {
			dateError = "date_past";
			return;
		}

		// name, subject, description, expires_at
		taskmgr.create(
			taskData.name,
			taskData.subject,
			(taskData.desc || ""),
			tempDate.getTime().toString()
		);

		dispatch("modalClose");
	}

	//console.log(subj.get_batch())
</script>

<!-- svelte-ignore a11y-missing-content -->
{#await setup()}
<div class="loading loading-lg"></div>
{:then dict}

<div class="modal modal-sm active" style="z-index: 999!important;">
	<a class="modal-overlay bg-primary" href="#modals-sizes" aria-label="Close" style="opacity: 30%"></a>
 	<div class="modal-container bg-dark" style="width: 60%; max-width: 80%!important;">
		<div class="modal-header" style="padding-bottom:0.5rem;">
		    <a class="btn btn-clear float-right" href="#modals-sizes" aria-label="Close" style="color: #fff" on:click={() => dispatch('modalClose')
		 	}></a>
	    	<div class="modal-title h5 text-left" style="color: #fff">{dict.add_task}</div>
		</div>
		{#await setupSubs()}
		<div class="loading loading-lg pb-2"></div>
		<p>{dict.loading}</p>
		{:then subs}

		{#if subs.length > 0}
		<div class="modal-body" style="padding-top:0.5rem;">
			<div class="content">
				<form class="col-mx-auto" style="width: 80%">

					<div class="form-group pb-2 {taskData.name ? 'has-success' : 'has-error'}">
						<label class="form-label" for="name">{dict.task_name}</label>
						<input bind:value={taskData.name} class="form-input text-center" id="name" type="text" placeholder={dict.write_essay} required>
						<p class="form-input-hint">{!taskData.name ? dict.field_empty : ''}</p>
					</div>
					<div class="form-group pb-2">
						<label class="form-label" for="description">{dict.task_description}</label>
						<textarea bind:value={taskData.desc} class="form-input text-center" id="description" placeholder={dict.essay_about_edu} rows="1" required></textarea>
					</div>

					<div class="form-group pb-2">
						<label class="form-label" for="subject">{dict.subject}</label>
						<select class="form-select" id="subject" bind:value={taskData.subject}>
							{#each subs as sub}
							<option>{sub.name}</option>
							{/each}
						</select>
					</div>

					<div class="form-group pb-2 {taskData.time && taskData.date && !dateError ? 'has-success' : 'has-error'}">
						<label class="form-label" for="due_date">{dict.due_date}</label>
						<div class="input-group">
							<input bind:value={taskData.time} class="form-input text-right" type="time">
							<input bind:this={taskData.jsDate} bind:value={taskData.date} class="form-input text-left input-group-addon" type="date">
						</div>
						<p class="form-input-hint">
							{!(taskData.time && taskData.date) ? dict.field_empty : ''}
							{!(taskData.time && taskData.date) && dateError ? '. ' : ''}
							{dateError ? dict[dateError] : ''}
						</p>
					</div>

				</form>
			</div>
		</div>
		<div class="modal-footer">
			{#if taskData.name && taskData.date && taskData.time}
			<button class="btn btn-primary" on:click={addTask}>{dict.add}</button>
			{:else}
			<button class="btn btn-primary" disabled>{dict.add}</button>
			{/if}

			<a class="btn btn-link" href="#modals-sizes" aria-label="Close" style="color: #fff" on:click={() => 
		 		dispatch('modalClose')
			}>{dict.close}</a>
		</div>
		{:else}
		<div class="empty bg-dark pt-2" style="height: 100%;">
			<p class="empty-title h5">{dict.no_subjects}</p>
			<p>{dict.add_subjects_at_settings}</p>
		</div>
		{/if}

		{:catch error}
		{error}
		<div class="empty bg-dark" style="height: 100%;">
			<div class="empty-icon">
				<i class="icon icon-3x icon-cross"></i>
			</div>
			<p class="empty-title h5">{dict.subjects_error}</p>
		</div>
		{/await}

	</div>
</div>
{:catch error}
<div class="empty bg-dark" style="height: 100%;">
	<div class="empty-icon">
		<i class="icon icon-3x icon-cross"></i>
	</div>
	<p class="empty-title h5">Could not load language data</p>
	<p class="empty-subtitle">Try searching for this issue on our GitHub repo. If you can not find a GitHub issue around this, please create one.</p>
</div>
{/await}

<style>
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
</style>
