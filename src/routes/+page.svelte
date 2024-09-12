<script>
	import { invoke } from '@tauri-apps/api';

	const date = new Date();
	let output = '';

	const handleClick = async () => {
		console.log('hello');
		try {
			const result = await invoke('run_bash_script', {
				scriptPath: './src/test.sh'
			});
			console.log(result);
		} catch (err) {
			console.error(err);
		}
	};
</script>

<div class="container mx-auto py-8">
	<h1 class="text-3xl font-bold">Cloud Tools</h1>
	<span>{date.getMinutes()} minutes</span>

	<button on:click={handleClick}> Run </button>
</div>

<style lang="postcss">
	:global(html) {
		background-color: theme(colors.gray.800);
		color: white;
	}
</style>
