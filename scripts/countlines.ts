/**
 * ------------------------------------------------------------
 * Script: countlines
 * Description: Counts lines in all files in given path
 * Author: @Bavfalcon9
 * ------------------------------------------------------------
 */
import * as path from "https://deno.land/std@0.97.0/path/mod.ts";
import { walkSync } from "https://deno.land/std@0.97.0/fs/walk.ts";

export function countlines(dir: string, iter: number = 0): [number, number] {
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
				console.log(`${file.path.replace(Deno.cwd(), '')} -> { %c${countwo.toLocaleString()}, %c${countcm.toLocaleString()}%c }`, "color: #58edef", "color: #f44949", "color: initial");
			}
			continue;
		}
	} catch (e) {
		console.error(e);
		console.log("Unknown Directory given: " + toCount);
	}
	return [ current, currentWith ];
}