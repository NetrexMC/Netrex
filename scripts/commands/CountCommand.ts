import * as path from "https://deno.land/std@0.97.0/path/mod.ts";
import { walkSync } from "https://deno.land/std@0.97.0/fs/walk.ts";
import Command from "./Command.ts";

export default class CountCommand extends Command {
	public label: string = "command.count";
	public name: string = "count";
	public aliases: string[] = [
		"countlines",
		"linecount"
	];
	public description: string = "Counts all lines in your project.";
	public usage: string = "{c} [path]\n {c} ./scripts\n {c}";

	public async execute(dir: string = "./src") {
		let lines = this.countlines(dir);
		console.log(`\n-> Total Lines: %c${lines[0].toLocaleString()}%c\n-> Total lines without comments: %c${lines[1].toLocaleString()}`, "color: #58edef", "color: initial", "color: #f44949");
	}

	public countlines(dir: string, iter: number = 0): [number, number] {
		const toCount = path.resolve(Deno.cwd(), dir);
		let currentWith = 0, current = 0;

		if (iter >= 100) {
			return [0,0];
		}

		try {
			Deno.statSync(toCount);
			for (const file of walkSync(toCount)) {
				if (file.isFile) {
					const p = path.resolve(toCount, file.name)
					const contents = new TextDecoder().decode(Deno.readFileSync(file.path));
					const countwo = contents.replace(/\\"|"(?:\\"|[^"])*"|(\/\/.*|\/\*[\s\S]*?\*\/)/gm, '').trim().split("\n").length;
					const countcm = contents.split('\n').length;
					current += countwo;
					currentWith += countcm;
					console.log(`${p.replace(Deno.cwd(), '')} -> { %c${countwo.toLocaleString()}, %c${countcm.toLocaleString()}%c }`, "color: #58edef", "color: #f44949", "color: initial");
				}
				continue;
			}
		} catch (e) {
			console.error(e);
			console.log("Unknown Directory given: " + toCount);
		}
		return [ current, currentWith ];
	}
}