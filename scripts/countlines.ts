/**
 * ------------------------------------------------------------
 * Script: countlines
 * Description: Counts lines in all files in given path
 * Author: @Bavfalcon9
 * ------------------------------------------------------------
 */
import * as path from "https://deno.land/std@0.97.0/path/mod.ts";

export async function countlines(dir: string): Promise<[number, number]> {
	const toCount = path.resolve(Deno.cwd(), dir);
	let currentWith = 0, current = 0;

	try {
		await Deno.stat(toCount);
		for await (const file of Deno.readDir(toCount)) {
			if (file.isDirectory) {
				let countdir = await countlines(path.resolve(toCount, file.name));
				current += countdir[0];
				currentWith += countdir[1];
			}
			if (file.isFile) {
				const p = path.resolve(toCount, file.name)
				const contents = new TextDecoder().decode(Deno.readFileSync(p));
				const countwo = contents.replace(/\*([^*]|[\r\n]|(\*+([^*/]|[\r\n])))*\*+/igm, '').trim().split("\n").length;
				const countcm = contents.split('\n').length;
				current += countwo;
				currentWith += countcm;
				console.log(`${p.replace(Deno.cwd(), '')} -> { %c${countwo.toLocaleString()}, %c${countcm.toLocaleString()}%c }`, "color: #58edef", "color: #f44949", "color: initial");
			}
		}
	} catch (e) {
		console.error(e);
		console.log("Unknown Directory given: " + toCount);
	}
	return [ current, currentWith ];
}