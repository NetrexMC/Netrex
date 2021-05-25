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
import EncapsulatedPacket from "./protocol/EncapsulatedPacket.ts";
import { OfflinePacketIds } from "./protocol/offline/OfflinePacket.ts";
import { openConnection, startSession } from "./RakHandler.ts";
import RakServer from "./RakServer.ts";
import { Stream } from "./util/Stream.ts";

export class RakConnection extends Connection {
	public state: ConnectionState;
	public address: Address;
	#server: RakServer;

	public constructor(address: Address, server: RakServer) {
		super();
		this.state = ConnectionState.Disconnected;
		this.address = address;
		this.#server = server;
	}

	public terminate(reason: string) {
		throw new Error("Method not implemented.");
	}
	public send(buffer: Uint8Array) {
		this.#server.send(this.address, buffer);
	}

	public recieve(buf: Stream) {
		const rakId = buf.readByte();

		if (this.state === ConnectionState.Disconnected) {
			// offline packets expected
			switch (rakId) {
				case OfflinePacketIds.OpenConnectRequest:
					openConnection(this, buf);
					break;
				case OfflinePacketIds.SessionInfo:
					startSession(this, buf);
					break;
			}
		}
	}

	public tick() {}
}
export default RakConnection;