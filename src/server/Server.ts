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
import MOTD from "https://raw.githubusercontent.com/RaptorsMC/RakNet/4754048cdbe58718c11bc52c4099c0906b51a8c2/lib/utils/MOTD.ts";
import { RakServer } from "../network/raknet/RakServer.ts"

export type ServerOptions =  {
	motd: MOTD,
	gamemode: "creative" | "survival",	
}

export const DefaultOptions: ServerOptions = {
	motd: new MOTD(),
	gamemode: "survival",
}

export default class Server {
	public raknet: RakServer;
	#opts: ServerOptions;

	public constructor(opts: Partial<ServerOptions>) {
		this.raknet = new RakServer();
		this.#opts = Object.assign(DefaultOptions, opts);
	}

	public start() {
		this.raknet.start();
	}

	public get options(): ServerOptions {
		return this.#opts;
	}
}