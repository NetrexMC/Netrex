/**
 * ------------------------------------------------------------
 * Script: Apply-headers
 * Description: Applies the license header to all source files.
 * Author: @Bavfalcon9
 * ------------------------------------------------------------
 */
//import Context from "./util/Context.ts";
import ResourceFile from "./util/ResourceFile.ts";

export function applyHeader(original: string): string {
	const license = new ResourceFile('license').contents;
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