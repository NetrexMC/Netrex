/**
 * ---------------------------------------------------
 * Util: ResourceFile
 * Description: Grabs a resource file from ./resources
 * Author: @Bavfalcon9
 * ---------------------------------------------------
 */

import * as path from "https://deno.land/std@0.97.0/path/mod.ts";

// locale
function parseJson(file: string): any {
	try {
		return JSON.parse(new TextDecoder().decode(Deno.readFileSync(file)));
	} catch {
		return {}
	}
}

export default class ResourceFile {
	#resource: string;

	public constructor(name: string) {
		const base = path.resolve(Deno.cwd(), './scripts/resources');
		const ids = parseJson(path.resolve(base, 'ids.json'));

		if (ids[name]) {
			this.#resource = path.resolve(base, ids[name]);
		} else {
			throw new Error("Resource not found.");
		}
	}

	/**
	 * Hard reads the resource files for contents.
	 */
	public get contents(): string {
		return new TextDecoder().decode(Deno.readFileSync(this.#resource));
	}

	/**
	 * Updates the contents of the resource file.
	 */
	public set contents(updated: string) {
		Deno.writeFileSync(this.#resource, new TextEncoder().encode(updated));
	}
}