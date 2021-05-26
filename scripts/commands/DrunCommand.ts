import Command from "./Command.ts";
import { exists } from "https://deno.land/std@0.97.0/fs/mod.ts";

export default class DrunCommand extends Command {
	public label: string = "command.drun";
	public name: string = "drun";
	public aliases: string[] = [
		"denorun",
	];
	public description: string = "Runs a project with a import map by the name of \"import_map.json\" with all flags and --unstable.";
	public usage: string = "{c} [module = mod.ts]\n {c} ./index.ts\n {c} ./mod.ts\n {c}";

	public async execute(file: string = 'mod.ts') {
		try {
			if (!await exists(file)) {
				console.log("%c🤚 Could not find \"" + file + "\" in your current directory.", "color: #fce262");
				return;
			}
			if (!await exists('import_map.json')) {
				console.log("%cNo import map was found. Please add a valid import_map.json to your project directory.", "color: #fce262");
			}
			console.log("%c🔃 Running %c\"" + file + "\"!", "color: #1e7bed", "color: initial;");
			const proc = Deno.run({
				cmd: `deno run -A --unstable --import-map=import_map.json ${file}`.split(" "),
				stdout: "inherit",
				stderr: "inherit",
				stdin: "inherit"
			});
			if ((await proc.status()).success) {
				console.log("%c✅ Completed %c\"" + file + "\"!", "color: #19ea3c", "color: initial;");
			}
		} catch (e) {
			console.log("%cAn unknown error occurred, this could likely be missing permissions.", "color: #f44949");
		}
	}
}
