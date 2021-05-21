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
import Connection from "../common/Connection.ts";
import NetworkServer, { NetworkType } from "../NetworkServer.ts";
import RakConnection from "./RakConnection.ts";
import { Stream } from "./util/Stream.ts";

export default class RakServer extends NetworkServer {
	public serverType: NetworkType = NetworkType.RakNet;
	#connects: Map<string, RakConnection> = new Map();
	#kill: boolean = false;
	#socket?: Deno.DatagramConn;

	public async start(address: string, port: number) {
		this.#socket = Deno.listenDatagram({
			hostname: address,
			port,
			transport: 'udp'
		});
		while (!this.#kill) {
			try {
				const request = await this.#socket.receive(new Uint8Array(2048));
				const stream = new Stream(request[0]);
				const origin = Address.from(request[1]);

				if (!this.#connects.has(origin.token)) {
					const session = new RakConnection(origin);
					this.#connects.set(origin.token, session);
				}

				const session = this.#connects.get(origin.token);

				if (!session) {
					// we can't handle this.
					continue;
				}

				session.recieve(stream);
			} catch {}
		}

		for (let conn of this.connections) {
			conn.terminate("NetworkServer shutdown.");
		}
	}

	public stop() {
		this.#kill = true;
		this.#socket?.close();
	}

	public get connections(): Connection[] {
		return [...this.#connects.values()];
	}
}