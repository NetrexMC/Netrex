import * as path from "https://deno.land/std@0.97.0/path/mod.ts";
import Command from "./Command.ts";

export default class WorkInit extends Command {
	public label: string = "command.workinit";
	public name: string = "workinit";
	public aliases: string[] = [
		"workspace",
		"initspace",
		"vscodedefaults",
		"vscode"
	];
	public description: string = "Initializes a netrex github-like workspace.";
	public usage: string = "{c} [path]\n {c} ./src\n {c}";

	public async execute(rel: string = "./") {
		const p = path.resolve(Deno.cwd(), rel);
		try {
			Deno.mkdirSync(path.resolve(p, '.vscode'), {recursive: true});
			Deno.writeFileSync(
				path.resolve(p, '.vscode/settings.json'),
				new TextEncoder().encode(JSON.stringify(SETTINGS_JSON, null, 2))
			);
			console.log("Initialized workspace for vscode.")
		} catch {
			console.log("Workspace already initialized.");
		}
	}
}
const SETTINGS_JSON = {
	"editor.tabSize": 4,
	"editor.detectIndentation": false,
	"editor.insertSpaces": false,
	"deno.enable": true,
	"deno.unstable": true,
	"deno.lint": false,
	"deno.import_intellisense_origins": {
		"https://deno.land": true
	}
};