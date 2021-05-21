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
import Address from "../common/Address.ts";
import Connection, { ConnectionState } from "../common/Connection.ts";
import { Stream } from "./util/Stream.ts";

export class RakConnection extends Connection {
	public state: ConnectionState;
	public address: Address;

	public constructor(address: Address) {
		super();
		this.state = ConnectionState.Disconnected;
		this.address = address;
	}

	public terminate(reason: string) {
		throw new Error("Method not implemented.");
	}
	public send(buffer: Uint8Array) {
		throw new Error("Method not implemented.");
	}

	public recieve(buf: Stream) {
		const rakId = buf.readByte();

		if (this.state === ConnectionState.Disconnected) {
			
		}
	}

	public tick() {}
}
export default RakConnection;