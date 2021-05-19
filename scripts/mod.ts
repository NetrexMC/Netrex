/**
 * ------------------------------------------------------------
 * Script: mod.ts
 * Description: Main script CLI entry point, simple bootstrap
 * Author: @Bavfalcon9
 * ------------------------------------------------------------
 */
import { applyHeader } from "./applyheader.ts";
import * as path from "https://deno.land/std@0.97.0/path/mod.ts";

if (import.meta.main) {
	const args = Deno.args;
	switch (args[0]?.toLowerCase()) {
		case "headers":
		case "applyheaders":
		case "aph": {
			aph(args[1] ?? "./src");
			break;
		}
		default:
		case "help": {
			console.log("Unknown Command \"" + (args[0] ?? "??") + "\".");
			break;
		}
	}
}

function print(msg: string): void {

}

async function aph(dir: string): Promise<void> {
	const toFix = path.resolve(Deno.cwd(), dir);

	try {
		await Deno.stat(toFix);
		for await (const file of Deno.readDir(toFix)) {
			if (file.isDirectory) {
				await aph(path.resolve(toFix, file.name));
			}
			if (file.isFile) {
				const ph = path.resolve(toFix, file.name)
				const fileRead = new TextDecoder().decode(Deno.readFileSync(ph));
				const contents = applyHeader(fileRead);
				await Deno.writeFile(ph, new TextEncoder().encode(contents));
				console.log("Formatted: " + ph);
			}
		}
	} catch (e) {
		console.error(e);
		console.log("Unknown Directory given: " + toFix);
	}
}