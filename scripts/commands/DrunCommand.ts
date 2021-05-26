import Command from "./Command.ts";

export default class DrunCommand extends Command {
	public label: string = "command.drun";
	public name: string = "drun";
	public aliases: string[] = [
		"denorun",
	];
	public description: string = "Runs the Netrex automatically project if provided an important map";
	public usage: string = "{c} [module = mod.ts]\n {c} ./index.ts\n {c} ./mod.ts\n {c}";

	public async execute(file: string = 'mod.ts') {
		try {
			await Deno.stat(file);
			await Deno.stat('import_map.json').catch(_ => { // Redo this, i just didnt want a nested try-catch
				console.log("%cNo import map was found. Please add a valid import_map.json to your root directory", "color: #fce262");
			});
			Deno.run({
				cmd: `deno run -A --import-map=import_map.json ${file}`.split(" "),
				stdout: "piped"
			})
		} catch (e) {
			console.error("Error! Your project should have an import map");
		}
	}
}
