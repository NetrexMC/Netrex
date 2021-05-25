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
import Address from "./network/common/Address.ts";
import { NetworkEventType } from "./network/NetworkServer.ts";
import { RakServer } from "./network/raknet/RakServer.ts";

export class Server {
	#network: RakServer;

	public constructor() {
		this.#network = new RakServer();
	}

	public start(): void {
		this.#network.start();
		this.#network.channel.on(NetworkEventType.GamePacket, (address: Address, buf: Uint8Array) => {
			// out of encapsulated here.
			// read first byte
			const pk = buf[0];
			console.log("Got: " + pk)
		});
	}
}