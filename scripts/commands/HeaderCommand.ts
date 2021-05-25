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
		const license = atob("LyoqCiAqICBfICAgXyAgICAgIF8KICogfCAgfCB8ICAgIHwgfAogKiB8ICB8IHwgX19ffCB8XyBfIF9fIF9fX19fICBfXwogKiB8IC4gYCB8LyBfICBfX3wgJ19fLyBfICAvIC8KICogfCB8ICB8ICBfXy8gfF98IHwgfCAgX18vPiAgPAogKiB8X3wgX3xfX198X198X3wgIF9fXy9fL18gKgogKiBUaGlzIHByb2dyYW0gaXMgZnJlZSBzb2Z0d2FyZTogeW91IGNhbiByZWRpc3RyaWJ1dGUgaXQgYW5kL29yIG1vZGlmeQogKiBpdCB1bmRlciB0aGUgdGVybXMgb2YgdGhlIEdOVSBMZXNzZXIgR2VuZXJhbCBQdWJsaWMgTGljZW5zZSBhcyBwdWJsaXNoZWQgYnkKICogdGhlIEZyZWUgU29mdHdhcmUgRm91bmRhdGlvbiwgZWl0aGVyIHZlcnNpb24gMyBvZiB0aGUgTGljZW5zZSwgb3IKICogKGF0IHlvdXIgb3B0aW9uKSBhbnkgbGF0ZXIgdmVyc2lvbi4KICoKICogQGF1dGhvciBOZXRyZXggVGVhbQogKiBAbGluayBodHRwczovL2dpdGh1Yi5jb20vTmV0cmV4TUMKICoKICogqSBOZXRyZXggMjAyMCAtIDIwMjEKICov");
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