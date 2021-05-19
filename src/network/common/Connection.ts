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
import Address from "./Address.ts";

export default abstract class Connection {
	/**
	 * The address of the current connection
	 */
	public abstract get address(): Address;

	/**
	 * Kill the connection for any given reason.
	 */
	public abstract terminate(reason: string): any;

	/**
	 * Sends any payload to the connection
	 */
	public abstract send(buffer: Uint8Array): any;
}