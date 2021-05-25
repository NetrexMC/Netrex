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
// import Address from "../common/Address.ts";
// import Connection from "../common/Connection.ts";
import NetworkServer, { NetworkEventType, NetworkType } from "../NetworkServer.ts";
import {
	BinaryStream as Stream
} from 'https://raw.githubusercontent.com/RaptorsMC/BinaryUtils/master/mod.ts';
import { default as CommonAddr } from "../common/Address.ts";
import { EventEmitter } from "https://deno.land/std@0.97.0/node/events.ts";


import { Listener, Connection, MOTD, Address, EncapsulatedPacket } from "https://raw.githubusercontent.com/RaptorsMC/RakNet/4754048cdbe58718c11bc52c4099c0906b51a8c2/mod.ts";

function cadd2Rak(add: CommonAddr): Address {
	return Address.from(add.toDenoAddr());
}

function rakadd2C(add: Address): CommonAddr {
	return CommonAddr.from(add.toDenoAddr());
}
export class RakServer extends NetworkServer {
	public serverType: NetworkType = NetworkType.RakNet;
	public channel: EventEmitter = new EventEmitter();
	#listener: Listener = new Listener();

	public constructor() {
		super();
		this.#listener.events.on('encapsulatedPacket', (add: Address, pk: EncapsulatedPacket) => {
			this.channel.emit(NetworkEventType.GamePacket, rakadd2C(add), pk.buffer);
		});
		this.#listener.events.on('connectionDestroyed', (address: Address, reason: string) => {
			this.channel.emit(NetworkEventType.Disconnect, rakadd2C(address));
	   	});
	}

	public async start() {
		this.#listener.listen();
	}

	public stop() {
		this.#listener.stop();
	}

	public send(address: CommonAddr, stream: Uint8Array) {
		this.#listener.sendBuffer(cadd2Rak(address), new Stream(stream).buffer);
	}

	public get connections(): Connection[] {
		return [...this.#listener.connections.values()];
	}
}