// This file is responsible for RakNek connections
import { ConnectionState } from "../common/Connection.ts";
import RakConnection from "./RakConnection.ts";
import { Stream } from "./util/Stream.ts";

export const MAGIC = '\x00\xff\xff\x00\xfe\xfe\xfe\xfe\xfd\xfd\xfd\xfd\x12\x34\x56\x78';

export function openConnection(connection: RakConnection, stream: Stream) {
	if (connection.state !== ConnectionState.Disconnected) {
		// can't handle unconnected packet.
		return;
	}

	const magic = stream.read(16);
	const protocol = stream.readByte();

	if (protocol < 10) {
		connection.terminate("Invalid Protocol");
		return;
	}
}