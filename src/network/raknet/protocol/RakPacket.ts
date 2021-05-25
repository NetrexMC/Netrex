/**
 *  _   _      _
 * | \ | |    | |
 * |  \| | ___| |_ _ __ _____  __
 * | . ` |/ _ \ __| '__/ _ \ \/ /
 * | |\  |  __/ |_| | |  __/>  <
 * |_| \_|\___|\__|_|  \___/_/\_\
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * @author Netrex Team
 * @link https://github.com/NetrexMC
 *
 * © Netrex 2020 - 2021
 */
import { Stream } from "../util/Stream.ts";

// This is for when TS decides to add static methods in interfaces.
// export interface ServerBound {
// 	static from(stream: Stream): any;
// }

export interface ClientBound {
	parse(): Stream;
}

export interface ServerBound {
	from(stream: Stream): any;
}

export default abstract class RakPacket {
	public abstract readonly id: number;
}