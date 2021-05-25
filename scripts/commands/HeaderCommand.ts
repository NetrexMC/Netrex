import * as path from "https://deno.land/std@0.97.0/path/mod.ts";
import ResourceFile from "../util/ResourceFile.ts";
import Command from "./Command.ts";

export default class HeaderCommand extends Command {
	public label: string = "command.header";
	public name: string = "header";
	public aliases: string[] = [
		"headers",
		"applyheaders"
	];
	public description: string = "Applies headers to all files with given name.";
	public usage: string = "{c} [path]";

	public async execute(dir: string = "./src") {
		const toFix = path.resolve(Deno.cwd(), dir);

		try {
			await Deno.stat(toFix);
			for await (const file of Deno.readDir(toFix)) {
				if (file.isDirectory) {
					await this.execute(path.resolve(toFix, file.name));
				}
				if (file.isFile) {
					const ph = path.resolve(toFix, file.name)
					const fileRead = new TextDecoder().decode(Deno.readFileSync(ph));
					const contents = this.applyHeader(fileRead);
					await Deno.writeFile(ph, new TextEncoder().encode(contents));
					if (fileRead === contents) {
						console.log("%c🚀 Skipped: %c" + ph, "color: #fce262", "color: initial;");
					} else {
						console.log("%c✅ Formatted: %c" + ph, "color: #19ea3c", "color: initial;");
					}
				}
			}
		} catch (e) {
			console.error(e);
			console.log("Unknown Directory given: " + toFix);
		}
	}

	public applyHeader(original: string): string {
		const license = new TextDecoder().decode(HEADER);
		const insertable = "/**\n" + license.split('\n').map(line => ((line.trim().length > 0) ? " * " + line : " *")).join('\n') + "\n */";

		if (original.substr(0, 3) === '/**') {
			return original;
		}
		if (original.includes(insertable)) {
			//context.warn("Duplicate write prevented on file: " + context.current.name);
			return original;
		} else {
			return insertable + "\n" + original;
		}
	}
}

export const HEADER = new Uint8Array([32, 95, 32, 32, 32, 95, 32, 32, 32, 32, 32, 32, 95, 13, 10,
	124, 32, 92, 32, 124, 32, 124, 32, 32, 32, 32, 124, 32, 124, 13,
	10, 124, 32, 32, 92, 124, 32, 124, 32, 95, 95, 95, 124, 32, 124,
	95, 32, 95, 32, 95, 95, 32, 95, 95, 95, 95, 95, 32, 32, 95,
	95, 13, 10, 124, 32, 46, 32, 96, 32, 124, 47, 32, 95, 32, 92,
	32, 95, 95, 124, 32, 39, 95, 95, 47, 32, 95, 32, 92, 32, 92,
	47, 32, 47, 13, 10, 124, 32, 124, 92, 32, 32, 124, 32, 32, 95,
	95, 47, 32, 124, 95, 124, 32, 124, 32, 124, 32, 32, 95, 95, 47,
	62, 32, 32, 60, 13, 10, 124, 95, 124, 32, 92, 95, 124, 92, 95,
	95, 95, 124, 92, 95, 95, 124, 95, 124, 32, 32, 92, 95, 95, 95,
	47, 95, 47, 92, 95, 92, 13, 10, 13, 10, 84, 104, 105, 115, 32,
	112, 114, 111, 103, 114, 97, 109, 32, 105, 115, 32, 102, 114, 101, 101,
	32, 115, 111, 102, 116, 119, 97, 114, 101, 58, 32, 121, 111, 117, 32,
	99, 97, 110, 32, 114, 101, 100, 105, 115, 116, 114, 105, 98, 117, 116,
	101, 32, 105, 116, 32, 97, 110, 100, 47, 111, 114, 32, 109, 111, 100,
	105, 102, 121, 13, 10, 105, 116, 32, 117, 110, 100, 101, 114, 32, 116,
	104, 101, 32, 116, 101, 114, 109, 115, 32, 111, 102, 32, 116, 104, 101,
	32, 71, 78, 85, 32, 76, 101, 115, 115, 101, 114, 32, 71, 101, 110,
	101, 114, 97, 108, 32, 80, 117, 98, 108, 105, 99, 32, 76, 105, 99,
	101, 110, 115, 101, 32, 97, 115, 32, 112, 117, 98, 108, 105, 115, 104,
	101, 100, 32, 98, 121, 13, 10, 116, 104, 101, 32, 70, 114, 101, 101,
	32, 83, 111, 102, 116, 119, 97, 114, 101, 32, 70, 111, 117, 110, 100,
	97, 116, 105, 111, 110, 44, 32, 101, 105, 116, 104, 101, 114, 32, 118,
	101, 114, 115, 105, 111, 110, 32, 51, 32, 111, 102, 32, 116, 104, 101,
	32, 76, 105, 99, 101, 110, 115, 101, 44, 32, 111, 114, 13, 10, 40,
	97, 116, 32, 121, 111, 117, 114, 32, 111, 112, 116, 105, 111, 110, 41,
	32, 97, 110, 121, 32, 108, 97, 116, 101, 114, 32, 118, 101, 114, 115,
	105, 111, 110, 46, 13, 10, 13, 10, 64, 97, 117, 116, 104, 111, 114,
	32, 78, 101, 116, 114, 101, 120, 32, 84, 101, 97, 109, 13, 10, 64,
	108, 105, 110, 107, 32, 104, 116, 116, 112, 115, 58, 47, 47, 103, 105,
	116, 104, 117, 98, 46, 99, 111, 109, 47, 78, 101, 116, 114, 101, 120,
	77, 67, 13, 10, 13, 10, 194, 169, 32, 78, 101, 116, 114, 101, 120,
	32, 50, 48, 50, 48, 32, 45, 32, 50, 48, 50,
	49
]);