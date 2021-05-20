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
class Address {
	#ip: string;
	#port: number;
	#version: 4 | 6;

	/**
	 * Get the Address instance from a Deno.NetAddr instance
	 * @param addr
	 */
	public static from(addr: Deno.Addr): Address {
		 addr = (addr as Deno.NetAddr);
		 let exp: RegExp = /^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$/gm;
		 let type: 4|6 = (addr.hostname.match(exp) === null) ? 4 : 6;
		 let port: number = addr.port;
		 return new Address(addr.hostname, port, type);
	}

	constructor(ip: string, port: number, version: 4|6 = 4) {
		 this.#ip = ip;
		 this.#port = port;
		 this.#version = version;
	}

	/**
	 * Gets the ip of the address
	 */
	public get ip(): string {
		 return this.#ip;
	}

	/**
	 * Gets the port of the address
	 */
	public get port(): number {
		 return this.#port;
	}

	/**
	 * Gets the protocolversion for the address
	 */
	public get version(): number {
		 return this.#version;
	}

	/**
	 * Gets the protocol for the address as a string
	 * @yields 0.0.0.0v4
	 */
	public get protocol(): string {
		 return this.ip + 'v' + this.version;
	}

	/**
	 * Gets the token for the address.
	 * @yields 0.0.0.0:19283
	 */
	public get token(): string {
		 return this.ip + ':' + this.port;
	}

	/**
	 * Converts to deno address for sending.
	 */
	public toDenoAddr(): Deno.Addr {
		 return {
			  hostname: this.ip,
			  port: this.port,
			  transport: 'udp'
		 }
	}
}
export default Address;