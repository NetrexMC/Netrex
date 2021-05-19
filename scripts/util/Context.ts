/**
 * ------------------------------------------------------------
 * Script: Context
 * Description: Script Context
 * Author: @Bavfalcon9
 * ------------------------------------------------------------
 */
import ResourceFile from "./ResourceFile.ts";

export interface DefaultContext {
	name: string;
	resources: ResourceFile[];
}

export default class Context<T = DefaultContext> {
	#context: T;
	#previous?: Context<T>;
	public constructor(context: T, previous?: Context<T>) {
		this.#context = context;
		this.#previous = previous;
	}

	public warn(message: string): void {
	}

	public info(message: string): void {}

	public get current(): T {
		return this.#context;
	}

	public get previous(): Context<T> | undefined {
		return this.#previous;
	}
}